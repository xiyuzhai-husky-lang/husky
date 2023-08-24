mod db;

use self::db::*;

pub struct DevComptime<Task: husky_task::IsTask> {
    db: ComptimeDb,
    storage: DevComptimeStorage<Task>,
    linkage_table: dashmap::DashMap<DevLinkageKey<Task>, DevLinkage<Task>>,
    // linkage_table: LinkageTable,
}

impl DevComptime {
    pub fn db(&self) -> &ComptimeDb {
        &self.db
    }
}
