use super::event::Event;
use crate::{
    convert::from_lsp_types, convert::to_lsp_types, lsp_ext, lsp_utils::is_cancelled,
    server::Server, task::Task, Result,
};
mod handlers;

pub(crate) fn handle_event(server: &mut Server, event: Event) -> Result<()> {
    let loop_start = std::time::Instant::now();
    handle_specifics(server, event, loop_start)?;
    handle_aftermath(server)?;
    return Ok(());

    fn handle_specifics(
        server: &mut Server,
        event: Event,
        loop_start: std::time::Instant,
    ) -> Result<()> {
        match event {
            Event::Lsp(msg) => handle_lsp::handle_lsp(server, msg, loop_start)?,
        }
        return Ok(());
    }

    fn handle_aftermath(server: &mut Server) -> Result<()> {
        let is_state_changed = server.process_changes();

        if is_state_changed {
            if server.config.enable_semantic_tokens_refresh() {
                server
                    .comm
                    .send_request::<lsp_types::request::SemanticTokensRefesh>((), |_, _| ());
            }
        }

        if is_state_changed || server.live_docs.take_changes() {
            if server.config.enable_diagnostics() {
                update_diagnostics(server)
            }
        }

        for file_diagnostics in server.ide_db_proxy.drain_diagnostic_changes() {
            let url = file_id_to_url(&server.vfs.read(), file_diagnostics.file_id);
            // let diagnostics = server
            //     .diagnostics
            //     .diagnostics_for(file_id)
            //     .cloned()
            //     .collect();
            let version = from_lsp_types::vfs_path(&url)
                .map(|path| server.live_docs.get(&path).map(|it| it.version))
                .unwrap_or_default();

            // ep!(diagnostics);
            server
                .comm
                .send_notification::<lsp_types::notification::PublishDiagnostics>(
                    lsp_types::PublishDiagnosticsParams {
                        uri: url,
                        diagnostics: file_diagnostics
                            .diagnostics
                            .into_iter()
                            .map(to_lsp_types::to_diagnostic)
                            .collect(),
                        version,
                    },
                );
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

        return Ok(());
    }

    fn update_diagnostics(server: &mut Server) {
        let subscriptions = server
            .live_docs
            .iter()
            .map(|path| server.vfs.read().get_file_id(path).unwrap())
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

mod handle_lsp;

pub(crate) fn file_id_to_url(vfs: &vfs::Vfs, id: vfs::FileID) -> lsp_types::Url {
    let path = vfs.get_file_path(id);
    let path = path.as_path().unwrap();
    to_lsp_types::url_from_abs_path(path)
}
