use common::*;

use husky_lang_compile_time::*;

use file::FileQueryGroup;

use crate::server::client_comm::ClientCommunicator;

pub(crate) fn send_updates(db: &HuskyLangCompileTime, comm: &ClientCommunicator) {
    db.module_iter().for_each(|module| {
        db.diagnostic_reserve(module).release(|diagnostics| {
            if let Some(file_id) = db.module_to_file_id(module).ok() {
                comm.send_diagnostics(db.url(file_id), batch_into!(diagnostics), None);
            }
        })
    });
}
