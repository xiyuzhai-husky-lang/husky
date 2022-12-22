use crate::*;
use expect_test::expect_file;
use husky_entity_path::EntityPathJar;

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
    DB::expect_test_packages_debug_result("package_manifest", TomlAstDb::package_manifest_ast)
}
