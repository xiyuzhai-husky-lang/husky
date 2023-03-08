use crate::*;

use husky_toml_token::TomlTokenJar;
use husky_vfs::*;
use husky_word::WordJar;
use salsa::{Database, ParallelDatabase, Snapshot};

#[salsa::db(WordJar, VfsJar, TomlTokenJar, TomlAstJar)]
#[derive(Default)]
struct DB {
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

#[test]
fn manifest_ast_works() {
    DB::default().vfs_expect_test_debug("package_manifest", TomlAstDb::package_manifest_ast)
}
