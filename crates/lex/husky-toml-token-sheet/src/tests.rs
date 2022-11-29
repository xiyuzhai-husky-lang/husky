use crate::*;
use husky_package_path::{PackagePathDb, PackagePathJar};
use husky_toolchain::ToolchainJar;
use husky_word::WordJar;

#[salsa::db(WordJar, TomlTokenSheetJar, PackagePathJar, ToolchainJar)]
#[derive(Default)]
struct MimicDB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for MimicDB {}

#[test]
fn builtin_library_toml_token_sheets() {
    let db = MimicDB::default();
    let package_path_menu = db.package_path_menu();
    db.toml_token_sheet(package_path_menu.core());
}
