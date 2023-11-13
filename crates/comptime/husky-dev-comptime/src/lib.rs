use std::path::Path;

use husky_coword::Name;
use husky_task::{helpers::DevLinkTime, linkage::IsLinktime, DevComptimeDb, IsTask};
use husky_vfs::{CrateKind, CratePath, PackagePath, VfsDb};

pub struct DevComptime<Task: IsTask> {
    db: DevComptimeDb<Task>,
    target: DevComptimeTarget,
    linktime: DevLinkTime<Task>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = VfsDb)]
pub enum DevComptimeTarget {
    None,
    SingleCrate(CratePath),
}

impl<Task: IsTask> DevComptime<Task> {
    pub fn new(target_crate_path: &Path) -> Self {
        let db: DevComptimeDb<Task> = Default::default();
        let toolchain = match db.current_toolchain() {
            Ok(toolchain) => toolchain,
            Err(_) => todo!(),
        };
        let target_package_path = match PackagePath::new_local_or_toolchain_package(
            &db,
            toolchain,
            Name::from_ref(&db, "mnist-classifier").unwrap(),
            target_crate_path,
        ) {
            Ok(package_path) => package_path,
            Err(_e) => todo!(),
        };
        let target_crate_path = CratePath::new(&db, target_package_path, CrateKind::Main);
        Self {
            linktime: IsLinktime::new_linktime(target_crate_path, &db),
            target: DevComptimeTarget::SingleCrate(target_crate_path),
            db,
        }
    }

    pub fn target(&self) -> DevComptimeTarget {
        self.target
    }
}

impl<Task: IsTask> DevComptime<Task> {
    pub fn db(&self) -> &DevComptimeDb<Task> {
        &self.db
    }
}

impl<Task: IsTask> Default for DevComptime<Task>
where
    DevLinkTime<Task>: Default,
{
    fn default() -> Self {
        Self {
            target: DevComptimeTarget::None,
            db: Default::default(),
            linktime: Default::default(),
        }
    }
}
