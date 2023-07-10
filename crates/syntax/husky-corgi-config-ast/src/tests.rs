pub(crate) use husky_vfs::test_utils::*;

use crate::*;
use husky_coword::CowordJar;
use husky_toml_token::TomlTokenJar;
use husky_vfs::*;
use salsa::{Database, ParallelDatabase, Snapshot};

#[salsa::db(CowordJar, VfsJar, TomlTokenJar, TomlAstJar, CorgiConfigAstJar)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl Database for DB {}

impl ParallelDatabase for DB {
    fn snapshot(&self) -> Snapshot<Self> {
        Snapshot::new(DB {
            storage: self.storage.snapshot(),
        })
    }
}
