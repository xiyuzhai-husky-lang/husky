pub mod db;

use std::path::Path;

use self::db::*;
use husky_task::{helpers::DevLinkTime, linkage::IsLinkTime, IsTask};
use husky_vfs::{CratePath, DiffPathBuf};

pub struct DevComptime<Task: IsTask> {
    db: DevComptimeDb,
    linktime: DevLinkTime<Task>,
}

impl<Task: IsTask> DevComptime<Task> {
    pub fn new(target_crate: &Path) -> Self {
        let db = Default::default();
        Self {
            db,
            linktime: IsLinkTime::new_linkage_table(todo!(), todo!()),
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
