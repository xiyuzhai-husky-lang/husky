pub mod db;

use std::path::Path;

use self::db::*;
use husky_coword::Name;
use husky_task::{helpers::DevLinkTime, linkage::IsLinkTime, IsTask};
use husky_vfs::{CrateKind, CratePath, DiffPathBuf, PackagePath, VfsDb};

pub struct DevComptime<Task: IsTask> {
    db: DevComptimeDb,
    linktime: DevLinkTime<Task>,
}

impl<Task: IsTask> DevComptime<Task> {
    pub fn new(target_crate_path: &Path) -> Self {
        let db: DevComptimeDb = Default::default();
        let toolchain = match db.current_toolchain() {
            Ok(toolchain) => toolchain,
            Err(_) => todo!(),
        };
        let package_path = match PackagePath::new_local_or_toolchain_package(
            &db,
            toolchain,
            Name::from_ref(&db, "mnist-classifier").unwrap(),
            target_crate_path,
        ) {
            Ok(package_path) => package_path,
            Err(e) => todo!(),
        };
        let crate_path = CratePath::new(&db, package_path, CrateKind::Main);
        Self {
            linktime: IsLinkTime::new_linkage_table(crate_path, &db),
            db,
        }
    }
}

impl<Task: IsTask> DevComptime<Task> {
    pub fn db(&self) -> &DevComptimeDb {
        &self.db
    }
}

impl<Task: IsTask> Default for DevComptime<Task>
where
    DevLinkTime<Task>: Default,
{
    fn default() -> Self {
        Self {
            db: Default::default(),
            linktime: Default::default(),
        }
    }
}
