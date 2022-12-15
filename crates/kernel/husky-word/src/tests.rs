use crate::*;

#[salsa::db(WordJar)]
#[derive(Default)]
pub struct DB {
    storage: salsa::Storage<DB>,
}

impl salsa::Database for DB {}

impl salsa::ParallelDatabase for DB {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(DB {
            storage: self.storage.snapshot(),
        })
    }
}
