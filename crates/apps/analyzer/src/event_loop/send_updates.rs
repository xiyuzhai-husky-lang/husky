use compile_time_db::*;

use file::FileQueryGroup;

use crate::server::client_comm::ClientCommunicator;

macro_rules! batch_into {
    ($e:expr) => {{
        $e.iter().map(|d| d.clone().into()).collect()
    }};
}

pub(crate) fn send_updates(db: &HuskyLangCompileTime, comm: &ClientCommunicator) {
    db.module_iter().for_each(|module| {
        db.diagnostic_reserve(module).release(|diagnostics| {
            if let Some(file_id) = db.module_file(module).ok() {
                comm.send_diagnostics(db.url(file_id), batch_into!(diagnostics), None);
            }
        })
    });
}
