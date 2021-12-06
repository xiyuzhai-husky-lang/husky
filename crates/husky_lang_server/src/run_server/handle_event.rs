use base_db::SourceDatabaseExt;
use common::ep;

use super::{
    dispatch::{NotificationDispatcher, RequestDispatcher},
    event::Event,
};
use crate::{
    convert::from_lsp_types,
    convert::to_lsp_types,
    lsp_ext,
    lsp_utils::{self, is_cancelled},
    reload,
    server::live_docs::DocumentData,
    server::Server,
    task::{self, PrimeCachesProgress, Task},
    Result,
};
mod handlers;

pub(crate) fn handle_event(server: &mut Server, event: Event) -> Result<()> {
    let loop_start = std::time::Instant::now();
    trace_pre_handle_event(server.taskpool.handle.len(), &event);
    let was_quiescent = server.is_quiescent();
    handle_specifics(server, event, loop_start);
    handle_aftermath(server, was_quiescent, loop_start);
    return Ok(());

    fn trace_pre_handle_event(task_queue_len: usize, event: &Event) {
        tracing::info!("handle_event({:?})", event);
        if task_queue_len > 0 {
            tracing::info!("task queue len: {}", task_queue_len);
        }
    }

    fn handle_specifics(
        server: &mut Server,
        event: Event,
        loop_start: std::time::Instant,
    ) -> Result<()> {
        match event {
            Event::Lsp(msg) => handle_lsp::handle_lsp(server, msg, loop_start)?,
            Event::Task(task) => handle_tasks_as_many_as_possible(server, task)?,
            Event::Vfs(msg) => handle_vfs(msg)?,
            Event::Flycheck(msg) => handle_flycheck(msg)?,
        }
        return Ok(());

        fn handle_tasks_as_many_as_possible(server: &mut Server, mut task: Task) -> Result<()> {
            let _p = profile::span("Server::handle_event/task");
            let mut prime_caches_progresses = Vec::new();
            loop {
                handle_task(server, task, &mut prime_caches_progresses);
                // Coalesce multiple task events into one loop turn
                task = match server.taskpool.receiver.try_recv() {
                    Ok(task) => task,
                    Err(_) => break,
                };
            }

            for progress in prime_caches_progresses {
                if let PrimeCachesProgress::End { cancelled } = progress {
                    server.prime_caches_queue.opn_completed(());
                    if cancelled {
                        server.prime_caches_queue.request_op();
                    }
                }

                let (state, message, fraction) = summarize_progress(progress);
                server.report_progress("Indexing", state, message, Some(fraction));
            }

            return Ok(());

            fn handle_task(
                server: &mut Server,
                task: Task,
                prime_caches_progresses: &mut Vec<PrimeCachesProgress>,
            ) {
                match task {
                    Task::Response(response) => server.comm.respond(response),
                    Task::Diagnostics(diagnostics_per_file) => {
                        for (file_id, diagnostics) in diagnostics_per_file {
                            server
                                .diagnostics
                                .set_native_diagnostics(file_id, diagnostics)
                        }
                    }
                    Task::PrimeCaches(progress) => match progress {
                        PrimeCachesProgress::Begin => prime_caches_progresses.push(progress),
                        PrimeCachesProgress::Report(_) => {
                            match prime_caches_progresses.last_mut() {
                                Some(last @ PrimeCachesProgress::Report(_)) => {
                                    // Coalesce subsequent update events.
                                    *last = progress;
                                }
                                _ => prime_caches_progresses.push(progress),
                            }
                        }
                        PrimeCachesProgress::End { .. } => prime_caches_progresses.push(progress),
                    },
                }
            }

            fn summarize_progress(
                progress: PrimeCachesProgress,
            ) -> (lsp_utils::Progress, Option<String>, f64) {
                match progress {
                    PrimeCachesProgress::Begin => (lsp_utils::Progress::Begin, None, 0.0),
                    PrimeCachesProgress::Report(report) => (
                        lsp_utils::Progress::Report,
                        Some(format!(
                            "{}/{} ({})",
                            report.n_done, report.n_total, report.on_crate
                        )),
                        lsp_utils::Progress::fraction(report.n_done, report.n_total),
                    ),
                    PrimeCachesProgress::End { .. } => (lsp_utils::Progress::End, None, 1.0),
                }
            }
        }

        fn handle_vfs(mut msg: vfs::loader::Message) -> Result<()> {
            todo!()
        }

        fn handle_flycheck(msg: flycheck::Message) -> Result<()> {
            todo!()
        }
    }

    fn handle_aftermath(
        server: &mut Server,
        was_quiescent: bool,
        loop_start: std::time::Instant,
    ) -> Result<()> {
        let is_state_changed = server.process_changes();
        if server.is_quiescent() {
            if !was_quiescent {
                for flycheck_handle in &server.flychecker.handles {
                    flycheck_handle.update();
                }
                if server.config.enable_cache_prefill() {
                    server.prime_caches_queue.request_op();
                }
            }

            if !was_quiescent || is_state_changed {
                if server.config.enable_semantic_tokens_refresh() {
                    server.semantic_tokens_cache.lock().clear();
                    server
                        .comm
                        .send_request::<lsp_types::request::SemanticTokensRefesh>((), |_, _| ());
                }
            }

            if !was_quiescent || is_state_changed || server.live_docs.take_changes() {
                if server.config.enable_diagnostics() {
                    update_diagnostics(server)
                }
            }
        }

        if let Some(diagnostic_changes) = server.diagnostics.take_changes() {
            for file_id in diagnostic_changes {
                let db = server.analysis_host.raw_database();
                let source_root = db.file_source_root(file_id);

                let url = file_id_to_url(&server.vfs.internal.read().vfs, file_id);
                let diagnostics = server
                    .diagnostics
                    .diagnostics_for(file_id)
                    .cloned()
                    .collect();
                let version = from_lsp_types::vfs_path(&url)
                    .map(|path| server.live_docs.get(&path).map(|it| it.version))
                    .unwrap_or_default();

                ep!(diagnostics);

                server
                    .comm
                    .send_notification::<lsp_types::notification::PublishDiagnostics>(
                        lsp_types::PublishDiagnosticsParams {
                            uri: url,
                            diagnostics,
                            version,
                        },
                    );
            }
        }
        if server.prime_caches_queue.should_start_op() {
            server.taskpool.handle.spawn_with_sender({
                let analysis = server.take_snapshot().analysis;
                move |sender| {
                    sender
                        .send(Task::PrimeCaches(task::PrimeCachesProgress::Begin))
                        .unwrap();
                    let res = analysis.prime_caches(|progress| {
                        let report = PrimeCachesProgress::Report(progress);
                        sender.send(Task::PrimeCaches(report)).unwrap();
                    });
                    sender
                        .send(Task::PrimeCaches(PrimeCachesProgress::End {
                            cancelled: res.is_err(),
                        }))
                        .unwrap();
                }
            });
        }

        let status = server.current_status();
        if server.last_reported_status.as_ref() != Some(&status) {
            server.last_reported_status = Some(status.clone());

            if let (lsp_ext::Health::Error, Some(message)) = (status.health, &status.message) {
                server
                    .comm
                    .show_message(lsp_types::MessageType::ERROR, message.clone());
            }
        }

        let loop_duration = loop_start.elapsed();
        if loop_duration > std::time::Duration::from_millis(100) {
            tracing::warn!("overly long loop turn: {:?}", loop_duration);
            server
                .comm
                .poke_husky_developer(format!("overly long loop turn: {:?}", loop_duration));
        }
        return Ok(());

        fn update_diagnostics(server: &mut Server) {
            let subscriptions = server
                .live_docs
                .iter()
                .map(|path| server.vfs.internal.read().vfs.get_file_id(path).unwrap())
                .collect::<Vec<_>>();

            tracing::trace!("updating notifications for {:?}", subscriptions);

            let snapshot = server.take_snapshot();
            server.taskpool.handle.spawn(move || {
                let diagnostics = subscriptions
                    .into_iter()
                    .filter_map(|file_id| {
                        handlers::gen_lsp_diagnostics(&snapshot, file_id)
                            .map_err(|err| {
                                if !is_cancelled(&*err) {
                                    tracing::error!("failed to compute diagnostics: {:?}", err);
                                }
                            })
                            .ok()
                            .map(|diags| (file_id, diags))
                    })
                    .collect::<Vec<_>>();
                Task::Diagnostics(diagnostics)
            });
        }
    }
}

mod handle_lsp;

pub(crate) fn file_id_to_url(vfs: &vfs::Vfs, id: vfs::FileID) -> lsp_types::Url {
    let path = vfs.get_file_path(id);
    let path = path.as_path().unwrap();
    to_lsp_types::url_from_abs_path(path)
}
