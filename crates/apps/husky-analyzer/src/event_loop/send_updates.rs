use crate::server::client_comm::ClientCommunicator;
use crate::*;

pub(crate) fn send_updates(_db: &AnalyzerDB, _comm: &ClientCommunicator) {
    eprintln!("todo: send updates")
    // db.module_iter().for_each(|module| {
    //     db.diagnostics_reserve(module).release(|diagnostics| {
    //         if let Some(file) = db.module_file(module).ok() {
    //             comm.send_diagnostics(db.url(file), diagnostics.iter().map(|d| d.clone().into()).collect(), None);
    //         }
    //     })
    // });
}
