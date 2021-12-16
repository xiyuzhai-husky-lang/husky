use common::*;

use husky_lang_db::HuskyLangDatabase;

use file::FileQuery;

use crate::server::client_comm::ClientCommunicator;
use diagnostic::DiagnosticQuery;
use scope::ScopeQuery;

pub(crate) fn send_updates(db: &HuskyLangDatabase, comm: &ClientCommunicator) {
    db.module_iter().for_each(|module| {
        db.diagnostic_reserve(module).drain(|diagnostics| {
            if let Some(file_id) = db.module_to_file_id(module) {
                comm.send_diagnostics(db.url(file_id), batch_into!(diagnostics), None);
            }
        })
    });
}
