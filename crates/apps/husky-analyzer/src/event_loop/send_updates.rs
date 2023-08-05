use crate::*;
use crate::{server::client_comm::ClientCommunicator, utils::log};
use husky_vfs::VfsDb;
use salsa::DebugWithDb;

pub(crate) fn send_updates(db: &AnalyzerDB, comm: &ClientCommunicator) {
    let live_packages = db.live_packages().unwrap();
    log!(
        "begin send_updates for live packages {:?}",
        &live_packages.debug(db)
    );
    log!("begin send_updates");
    for package_path in live_packages.iter() {
        log!(
            "begin send_updates for live package {:?}",
            &package_path.debug(db)
        );
        let probable_modules = db.collect_probable_modules(*package_path);
        log!("probable_modules are {:?}", &probable_modules.debug(db));
        for module_path in probable_modules {
            log!("begin send_updates for module {:?}", &module_path.debug(db));
            comm.send_diagnostics(db, module_path);
            log!("end");
        }
    }
    log!("finish send_updates");
}
