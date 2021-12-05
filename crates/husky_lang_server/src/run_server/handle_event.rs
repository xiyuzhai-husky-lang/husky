use base_db::SourceDatabaseExt;

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
            Event::Lsp(msg) => handle_lsp(server, msg, loop_start)?,
            Event::Task(task) => handle_tasks_as_many_as_possible(server, task)?,
            Event::Vfs(msg) => handle_vfs(msg)?,
            Event::Flycheck(msg) => handle_flycheck(msg)?,
        }
        return Ok(());

        fn handle_lsp(
            server: &mut Server,
            msg: lsp_server::Message,
            loop_start: std::time::Instant,
        ) -> Result<()> {
            match msg {
                lsp_server::Message::Request(req) => handle_lsp_request(server, loop_start, req)?,
                lsp_server::Message::Notification(notif) => {
                    handle_lsp_notification(server, notif)?;
                }
                lsp_server::Message::Response(resp) => handle_lsp_request_complete(server, resp),
            }
            return Ok(());

            fn handle_lsp_request(
                server: &mut Server,
                request_received: std::time::Instant,
                req: lsp_server::Request,
            ) -> Result<()> {
                server
                    .comm
                    .req_queue
                    .incoming
                    .register(req.id.clone(), (req.method.clone(), request_received));

                if server.shutdown_requested {
                    server.comm.respond(lsp_server::Response::new_err(
                        req.id,
                        lsp_server::ErrorCode::InvalidRequest as i32,
                        "Shutdown already requested.".to_owned(),
                    ));

                    return Ok(());
                }

                // Avoid flashing a bunch of unresolved references during initial load.
                if server.config.projects.is_empty() && !server.is_quiescent() {
                    server.comm.respond(lsp_server::Response::new_err(
                        req.id,
                        // FIXME: i32 should impl From<ErrorCode> (from() guarantees lossless conversion)
                        lsp_server::ErrorCode::ContentModified as i32,
                        "waiting for cargo metadata or cargo check".to_owned(),
                    ));
                    return Ok(());
                }

                RequestDispatcher {
                    req: Some(req),
                    server,
                }
                .on_sync_mut::<lsp_types::request::Shutdown>(|server, ()| {
                    server.shutdown_requested = true;
                    Ok(())
                })?
                .on_sync_mut::<lsp_ext::MemoryUsage>(handlers::handle_memory_usage)?
                .on_sync::<lsp_ext::JoinLines>(handlers::handle_join_lines)?
                .on_sync::<lsp_ext::OnEnter>(handlers::handle_on_enter)?
                .on_sync::<lsp_types::request::SelectionRangeRequest>(
                    handlers::handle_selection_range,
                )?
                .on_sync::<lsp_ext::MatchingBrace>(handlers::handle_matching_brace)?
                .on::<lsp_ext::AnalyzerStatus>(handlers::handle_analyzer_status)
                .on::<lsp_ext::SyntaxTree>(handlers::handle_syntax_tree)
                .on::<lsp_ext::ViewHir>(handlers::handle_view_hir)
                .on::<lsp_ext::ViewCrateGraph>(handlers::handle_view_crate_graph)
                .on::<lsp_ext::ViewItemTree>(handlers::handle_view_item_tree)
                .on::<lsp_ext::ParentModule>(handlers::handle_parent_module)
                .on::<lsp_ext::Runnables>(handlers::handle_runnables)
                .on::<lsp_ext::RelatedTests>(handlers::handle_related_tests)
                .on::<lsp_ext::InlayHints>(handlers::handle_inlay_hints)
                .on::<lsp_ext::CodeActionRequest>(handlers::handle_code_action)
                .on::<lsp_ext::CodeActionResolveRequest>(handlers::handle_code_action_resolve)
                .on::<lsp_ext::HoverRequest>(handlers::handle_hover)
                .on::<lsp_ext::ExternalDocs>(handlers::handle_open_docs)
                .on::<lsp_ext::OpenCargoToml>(handlers::handle_open_cargo_toml)
                .on::<lsp_ext::MoveItem>(handlers::handle_move_item)
                .on::<lsp_ext::WorkspaceSymbol>(handlers::handle_workspace_symbol)
                .on::<lsp_types::request::OnTypeFormatting>(handlers::handle_on_type_formatting)
                .on::<lsp_types::request::DocumentSymbolRequest>(handlers::handle_document_symbol)
                .on::<lsp_types::request::GotoDefinition>(handlers::handle_goto_definition)
                .on::<lsp_types::request::GotoDeclaration>(handlers::handle_goto_declaration)
                .on::<lsp_types::request::GotoImplementation>(handlers::handle_goto_implementation)
                .on::<lsp_types::request::GotoTypeDefinition>(handlers::handle_goto_type_definition)
                .on::<lsp_types::request::Completion>(handlers::handle_completion)
                .on::<lsp_types::request::ResolveCompletionItem>(
                    handlers::handle_completion_resolve,
                )
                .on::<lsp_types::request::CodeLensRequest>(handlers::handle_code_lens)
                .on::<lsp_types::request::CodeLensResolve>(handlers::handle_code_lens_resolve)
                .on::<lsp_types::request::FoldingRangeRequest>(handlers::handle_folding_range)
                .on::<lsp_types::request::SignatureHelpRequest>(handlers::handle_signature_help)
                .on::<lsp_types::request::PrepareRenameRequest>(handlers::handle_prepare_rename)
                .on::<lsp_types::request::Rename>(handlers::handle_rename)
                .on::<lsp_types::request::References>(handlers::handle_references)
                .on::<lsp_types::request::Formatting>(handlers::handle_formatting)
                .on::<lsp_types::request::RangeFormatting>(handlers::handle_range_formatting)
                .on::<lsp_types::request::DocumentHighlightRequest>(
                    handlers::handle_document_highlight,
                )
                .on::<lsp_types::request::CallHierarchyPrepare>(
                    handlers::handle_call_hierarchy_prepare,
                )
                .on::<lsp_types::request::CallHierarchyIncomingCalls>(
                    handlers::handle_call_hierarchy_incoming,
                )
                .on::<lsp_types::request::CallHierarchyOutgoingCalls>(
                    handlers::handle_call_hierarchy_outgoing,
                )
                .on::<lsp_types::request::SemanticTokensFullRequest>(
                    handlers::handle_semantic_tokens_full,
                )
                .on::<lsp_types::request::SemanticTokensFullDeltaRequest>(
                    handlers::handle_semantic_tokens_full_delta,
                )
                .on::<lsp_types::request::SemanticTokensRangeRequest>(
                    handlers::handle_semantic_tokens_range,
                )
                .on::<lsp_types::request::WillRenameFiles>(handlers::handle_will_rename_files)
                .on::<lsp_ext::Ssr>(handlers::handle_ssr)
                .finish();
                Ok(())
            }

            fn handle_lsp_notification(
                server: &mut Server,
                notif: lsp_server::Notification,
            ) -> Result<()> {
                NotificationDispatcher {
                    notif: Some(notif),
                    server,
                }
                .on::<lsp_types::notification::Cancel>(|this, params| {
                    let id: lsp_server::RequestId = match params.id {
                        lsp_types::NumberOrString::Number(id) => id.into(),
                        lsp_types::NumberOrString::String(id) => id.into(),
                    };
                    // this.cancel(id);
                    todo!();
                    Ok(())
                })?
                .on::<lsp_types::notification::WorkDoneProgressCancel>(|_this, _params| {
                    // Just ignore this. It is OK to continue sending progress
                    // notifications for this token, as the client can't know when
                    // we accepted notification.
                    Ok(())
                })?
                .on::<lsp_types::notification::DidOpenTextDocument>(handle_did_open_text_document)?
                .on::<lsp_types::notification::DidChangeTextDocument>(|this, params| {
                    todo!();
                })?
                .on::<lsp_types::notification::DidCloseTextDocument>(|this, params| {
                    todo!();
                })?
                .on::<lsp_types::notification::DidSaveTextDocument>(|this, params| {
                    todo!();
                })?
                .on::<lsp_types::notification::DidChangeConfiguration>(|this, _params| {
                    todo!();
                })?
                .on::<lsp_types::notification::DidChangeWatchedFiles>(|this, params| {
                    todo!();
                })?
                .finish();
                return Ok(());

                fn handle_did_open_text_document(
                    this: &mut Server,
                    params: lsp_types::DidOpenTextDocumentParams,
                ) -> Result<()> {
                    if let Ok(path) = from_lsp_types::vfs_path(&params.text_document.uri) {
                        this.live_docs
                            .insert(
                                path.clone(),
                                DocumentData::new(params.text_document.version),
                            )
                            .err()
                            .map(|_| tracing::error!("duplicate DidOpenTextDocument: {}", path));
                        this.vfs.internal.write().vfs.update_file_contents(
                            path,
                            Some(params.text_document.text.into_bytes()),
                        );
                    }
                    Ok(())
                }
            }

            fn handle_lsp_request_complete(server: &mut Server, response: lsp_server::Response) {
                let handler = server.comm.req_queue.outgoing.complete(response.id.clone());
                handler(server, response)
            }
        }

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
                if db.source_root(source_root).is_library {
                    // Only publish diagnostics for files in the workspace, not from crates.io deps
                    // or the sysroot.
                    // While theoretically these should never have errors, we have quite a few false
                    // positives particularly in the stdlib, and those diagnostics would stay around
                    // forever if we emitted them here.
                    continue;
                }

                let url = file_id_to_url(&server.vfs.internal.read().vfs, file_id);
                let diagnostics = server
                    .diagnostics
                    .diagnostics_for(file_id)
                    .cloned()
                    .collect();
                let version = from_lsp_types::vfs_path(&url)
                    .map(|path| server.live_docs.get(&path).map(|it| it.version))
                    .unwrap_or_default();

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
                        handlers::publish_diagnostics(&snapshot, file_id)
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
            todo!();
        }
    }
}

pub(crate) fn file_id_to_url(vfs: &vfs::Vfs, id: vfs::FileID) -> lsp_types::Url {
    let path = vfs.get_file_path(id);
    let path = path.as_path().unwrap();
    to_lsp_types::url_from_abs_path(path)
}
