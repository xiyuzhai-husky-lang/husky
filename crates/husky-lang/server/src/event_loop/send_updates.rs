use common::*;

use husky_lang_db::HuskyLangDatabase;

use crate::{convert, server::client_comm::ClientCommunicator};

pub(crate) fn send_updates(snapshot: &HuskyLangDatabase, comm: &ClientCommunicator) {
    use scope::{ScopeQuery, ScopeSalsaQuery};
    use std::ops::Deref;

    use diagnostic::DiagnosticQuery;
    snapshot.all_modules().into_iter().for_each(|module| {
        snapshot.diagnostic_reserve(module).drain(|diagnostics| {
            ep!(diagnostics.len());
            let diagnostics = diagnostics
                .into_iter()
                .map(|d| convert::to_lsp_types::to_diagnostic(d))
                .collect();
            if let Some(scope_source) = snapshot.deref().scope_source(module.scope_id) {
                let file_id = match scope_source {
                    scope::ScopeSource::Builtin(_) => todo!(),
                    scope::ScopeSource::WithinModule { file_id, .. } => file_id,
                    scope::ScopeSource::Module { file_id } => file_id,
                };
                comm.send_notification::<lsp_types::notification::PublishDiagnostics>(
                    lsp_types::PublishDiagnosticsParams {
                        uri: file::use_filepath(snapshot.deref(), file_id, |pth| {
                            convert::to_lsp_types::url_from_abs_path(pth)
                        }),
                        diagnostics,
                        version: None,
                    },
                );
            }
        })
    });
}
