mod db;

use self::db::*;

pub struct Comptime {
    db: ComptimeDb,
}

impl Comptime {
    pub fn db(&self) -> &ComptimeDb {
        &self.db
    }
}
