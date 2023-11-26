pub(crate) use husky_vfs::test_utils::*;

use crate::*;
use husky_coword::CowordJar;
use husky_toml_token::TomlTokenJar;
use husky_vfs::*;
use salsa::{Database, ParallelDatabase, Snapshot};

#[salsa::test_db(CowordJar, VfsJar, TomlTokenJar, TomlAstJar)]
struct DB;

#[test]
fn manifest_ast_works() {
    DB::default().vfs_expect_test_debug_with_db(
        |db, path| db.package_manifest_toml_ast_sheet(path),
        &VfsTestConfig::new("package_manifest_ast_sheet_sheet"),
    )
}
