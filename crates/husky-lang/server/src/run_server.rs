mod dispatch;
mod handle_lsp;

use common::*;

use std::error::Error;

use crate::server::{Server, ServerControlSignal};

pub fn run_server(connection: lsp_server::Connection) -> Result<(), Box<dyn Error + Sync + Send>> {
    let mut server = Server::new(connection.sender);
    while let Some(msg) = connection.receiver.recv().ok() {
        let loop_start = std::time::Instant::now();
        match handle_lsp::handle_lsp_msg(&mut server, msg, loop_start)? {
            ServerControlSignal::Normal => send_updates(&server)?,
            ServerControlSignal::Shutdown => return Ok(()),
        }
    }
    return Err("client exited without proper shutdown sequence".into());

    fn send_updates(server: &Server) -> Result<()> {
        use diagnostic::DiagnosticQuery;
        todo!();
        // server.db.diagnostics().drain();
        return Ok(());

        fn send_diagnostics(
            server: &Server,
            file_id: file::FileId,
            diagnostics: Vec<diagnostic::Diagnostic>,
        ) -> Result<()> {
            todo!()
            // use file::VirtualFileSystem;

            // let _p = profile::span("publish_diagnostics");
            // let line_map = server.db.line_map(file_id)?;

            // let diagnostics = diagnostics
            //     .into_iter()
            //     .map(|d| to_lsp_types::to_diagnostic(&line_map, d))
            //     .collect();

            // server
            //     .comm
            //     .send_notification::<lsp_types::notification::PublishDiagnostics>(
            //         lsp_types::PublishDiagnosticsParams {
            //             uri: to_lsp_types::url_from_abs_path(server.db.path(file_id)),
            //             diagnostics,
            //             version: None,
            //         },
            //     );
            // Ok(())
        }
    }
}
