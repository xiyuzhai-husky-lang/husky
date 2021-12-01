use std::time::Instant;

use super::{
    dispatch::{NotificationDispatcher, RequestDispatcher},
    event::Event,
};
use crate::{from_proto, server::live_docs::DocumentData, server::Server, task::Task, Result};

pub(crate) fn handle_event(server: &mut Server, event: Event) -> Result<()> {
    let loop_start = Instant::now();
    trace_pre_handle_event(server.taskpool.handle.len(), &event);
    let was_quiescent = server.is_quiescent();
    handle_specifics(server, event, loop_start);
    handle_aftermath(server, was_quiescent);
    return Ok(());

    fn trace_pre_handle_event(task_queue_len: usize, event: &Event) {
        tracing::info!("handle_event({:?})", event);
        if task_queue_len > 0 {
            tracing::info!("task queue len: {}", task_queue_len);
        }
    }

    fn handle_specifics(server: &mut Server, event: Event, loop_start: Instant) -> Result<()> {
        match event {
            Event::Lsp(msg) => handle_lsp(server, msg, loop_start)?,
            Event::Task(task) => handle_task(task)?,
            Event::Vfs(mut msg) => handle_vfs(msg)?,
            Event::Flycheck(mut msg) => handle_flycheck(msg)?,
        }
        return Ok(());

        fn handle_lsp(
            server: &mut Server,
            msg: lsp_server::Message,
            loop_start: Instant,
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
                request_received: Instant,
                req: lsp_server::Request,
            ) -> Result<()> {
                todo!()
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
                    if let Ok(path) = from_proto::vfs_path(&params.text_document.uri) {
                        this.live_docs
                            .insert(
                                path.clone(),
                                DocumentData::new(params.text_document.version),
                            )
                            .err()
                            .map(|_| tracing::error!("duplicate DidOpenTextDocument: {}", path));
                        this.vfs
                            .internal
                            .write()
                            .vfs
                            .set_file_contents(path, Some(params.text_document.text.into_bytes()));
                    }
                    Ok(())
                }
            }

            fn handle_lsp_request_complete(server: &mut Server, response: lsp_server::Response) {
                todo!()
            }
        }

        fn handle_task(task: Task) -> Result<()> {
            todo!()
        }

        fn handle_vfs(mut msg: vfs::loader::Message) -> Result<()> {
            todo!()
        }

        fn handle_flycheck(msg: flycheck::Message) -> Result<()> {
            todo!()
        }
    }

    fn handle_aftermath(server: &mut Server, was_quiescent: bool) {
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
                // Refresh semantic tokens if the client supports it.
                if server.config.enable_semantic_tokens_refresh() {
                    server.semantic_tokens_cache.lock().clear();
                    server
                        .sender
                        .send_request::<lsp_types::request::SemanticTokensRefesh>((), |_, _| ());
                }
            }

            if !was_quiescent || is_state_changed || server.live_docs.take_changes() {
                if server.config.enable_diagnostics() {
                    update_diagnostics(server)
                }
            }
        }
        todo!();

        fn update_diagnostics(server: &mut Server) {
            let subscriptions = server
                .live_docs
                .iter()
                .map(|path| server.vfs.internal.read().vfs.file_id(path).unwrap())
                .collect::<Vec<_>>();

            tracing::trace!("updating notifications for {:?}", subscriptions);

            let snapshot = server.snapshot();
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
            })
        }
    }
}
