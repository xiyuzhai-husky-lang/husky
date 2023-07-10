pub(crate) use husky_vfs::test_utils::*;

use crate::*;
use husky_coword::CowordJar;
use husky_toml_token::TomlTokenJar;
use husky_vfs::*;
use salsa::{Database, ParallelDatabase, Snapshot};

#[salsa::db(CowordJar, VfsJar, TomlTokenJar, TomlAstJar)]
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
    DB::default().vfs_expect_test_debug_with_db(
        "package_manifest_ast_sheet_sheet",
        TomlAstDb::package_manifest_toml_ast_sheet,
    )
}
