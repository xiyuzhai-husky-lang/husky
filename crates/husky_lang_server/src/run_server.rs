mod dispatch;
mod handle_lsp;

use common::*;

use std::error::Error;

use crate::{
    config::ServerConfig,
    convert::to_lsp_types,
    server::{Server, ServerControlSignal},
};

pub fn run_server(
    server_config: ServerConfig,
    connection: lsp_server::Connection,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let mut server = Server::new(connection.sender, server_config);
    while let Some(msg) = connection.receiver.recv().ok() {
        let loop_start = std::time::Instant::now();
        match handle_lsp::handle_lsp_msg(&mut server, msg, loop_start)? {
            ServerControlSignal::Normal => send_updates(&server)?,
            ServerControlSignal::Shutdown => return Ok(()),
        }
    }
    return Err("client exited without proper shutdown sequence".into());

    fn send_updates(server: &Server) -> Result<()> {
        server.db.on_diagnostic_change(&|file_id, diagnostics| {
            send_diagnostics(server, file_id, diagnostics)
        })?;
        return Ok(());

        fn send_diagnostics(
            server: &Server,
            file_id: vfs::SourceFileId,
            diagnostics: Vec<hir::Diagnostic>,
        ) -> Result<()> {
            use vfs::VirtualFileSystem;

            let _p = profile::span("publish_diagnostics");
            let line_map = server.db.line_map(file_id)?;

            let diagnostics = diagnostics
                .into_iter()
                .map(|d| to_lsp_types::to_diagnostic(&line_map, d))
                .collect();

            server
                .comm
                .send_notification::<lsp_types::notification::PublishDiagnostics>(
                    lsp_types::PublishDiagnosticsParams {
                        uri: to_lsp_types::url_from_abs_path(server.db.path(file_id)),
                        diagnostics,
                        version: None,
                    },
                );
            Ok(())
        }
    }
}
