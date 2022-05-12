use compile_time_db::*;

use file::FileQueryGroup;
use print_utils::ep;

use crate::server::client_comm::ClientCommunicator;

macro_rules! batch_into {
    ($e:expr) => {{
        $e.iter().map(|d| d.clone().into()).collect()
    }};
}

pub(crate) fn send_updates(db: &HuskyLangCompileTime, comm: &ClientCommunicator) {
    let modules: Vec<_> = db.module_iter().collect();
    eprintln!("modules: {:?}", modules);
    db.module_iter().for_each(|module| {
        db.diagnostics_reserve(module).release(|diagnostics| {
            if let Some(file) = db.module_file(module).ok() {
                ep!(module, file);
                comm.send_diagnostics(db.url(file), batch_into!(diagnostics), None);
            }
        })
    });
}
