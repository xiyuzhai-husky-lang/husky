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
            file_id: vfs::FileId,
            diagnostics: Vec<hir::Diagnostic>,
        ) -> Result<()> {
            let _p = profile::span("publish_diagnostics");
            let line_index = server.db.get_file_line_collection(file_id)?;

            let diagnostics = diagnostics
                .into_iter()
                .map(|d| lsp_types::Diagnostic {
                    range: to_lsp_types::range(&line_index, d.range),
                    severity: Some(to_lsp_types::to_diagnostic_severity(d.severity)),
                    code: Some(lsp_types::NumberOrString::String(
                        d.code.as_str().to_string(),
                    )),
                    code_description: None,
                    source: Some("husky-lang-server".to_string()),
                    message: d.message,
                    related_information: None,
                    tags: None,
                    data: None,
                })
                .collect();

            server
                .comm
                .send_notification::<lsp_types::notification::PublishDiagnostics>(
                    lsp_types::PublishDiagnosticsParams {
                        uri: to_lsp_types::url_from_abs_path(
                            server
                                .db
                                .get_vfs_path_from_file_id(file_id)
                                .as_path()
                                .expect(""),
                        ),
                        diagnostics,
                        version: None,
                    },
                );
            Ok(())
        }
    }
}
