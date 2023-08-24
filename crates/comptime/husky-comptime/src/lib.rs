mod db;

use self::db::*;

pub struct Comptime {
    db: ComptimeDb,
    // linkage_table: LinkageTable,
}

impl Comptime {
    pub fn db(&self) -> &ComptimeDb {
        &self.db
    }
}
