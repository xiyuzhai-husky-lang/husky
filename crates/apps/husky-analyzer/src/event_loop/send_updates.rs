use crate::server::client_comm::ClientCommunicator;
use crate::*;
use husky_vfs::VfsDb;
use salsa::DebugWithDb;

pub(crate) fn send_updates(db: &AnalyzerDB, comm: &ClientCommunicator) {
    let live_packages = db.live_packages().unwrap();
    for package_path in live_packages.iter() {
        let collect_probable_modules = db.collect_probable_modules(*package_path);
        for module_path in collect_probable_modules {
            comm.send_diagnostics(db, module_path)
        }
    }
    // db.module_iter().for_each(|module| {
    //     db.diagnostics_reserve(module).release(|diagnostics| {
    //         if let Some(file) = db.module_file(module).ok() {
    //             comm.send_diagnostics(db.url(file), diagnostics.iter().map(|d| d.clone().into()).collect(), None);
    //         }
    //     })
    // });
}
