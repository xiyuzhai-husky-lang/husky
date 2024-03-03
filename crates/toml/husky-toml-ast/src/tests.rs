pub(crate) use husky_vfs::test_utils::*;

use crate::*;
use husky_coword::CowordJar;
use husky_vfs::*;

#[salsa::db(CowordJar, VfsJar, husky_toml_token::jar::TomlTokenJar, TomlAstJar)]
struct DB;

#[test]
fn manifest_ast_works() {
    DB::vfs_expect_test_debug_with_db(
        |db, path| db.package_manifest_toml_ast_sheet(path),
        &VfsTestConfig::new(
            "package_manifest_ast_sheet_sheet",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::TOML,
        ),
    )
}
