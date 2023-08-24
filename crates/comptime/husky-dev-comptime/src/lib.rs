mod db;

use self::db::*;
use husky_task::{DevLinkageTable, IsTask};

pub struct DevComptime<Task: IsTask> {
    db: ComptimeDb,
    linkage_table: DevLinkageTable<Task>,
}

impl<Task: IsTask> DevComptime<Task> {
    pub fn db(&self) -> &ComptimeDb {
        &self.db
    }
}
