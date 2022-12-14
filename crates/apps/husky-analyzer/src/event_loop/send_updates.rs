use crate::server::client_comm::ClientCommunicator;
use crate::*;

macro_rules! batch_into {
    ($e:expr) => {{
        $e.iter().map(|d| d.clone().into()).collect()
    }};
}

pub(crate) fn send_updates(db: &AnalysisHost, comm: &ClientCommunicator) {
    todo!()
    // db.module_iter().for_each(|module| {
    //     db.diagnostics_reserve(module).release(|diagnostics| {
    //         if let Some(file) = db.module_file(module).ok() {
    //             comm.send_diagnostics(db.url(file), batch_into!(diagnostics), None);
    //         }
    //     })
    // });
}
