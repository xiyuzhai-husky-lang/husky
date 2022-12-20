use crate::*;
use expect_test::expect_file;
use husky_entity_path::EntityPathJar;
use husky_package_path::{PackagePathDb, PackagePathJar};

use husky_toml_token::TomlTokenJar;
use husky_toolchain::*;
use husky_vfs::*;
use husky_word::{WordJar};
use salsa::{Database, ParallelDatabase, Snapshot};


#[salsa::db(
    WordJar,
    ToolchainJar,
    PackagePathJar,
    EntityPathJar,
    TomlTokenJar,
    VfsJar,
    TomlAstJar
)]
#[derive(Default)]
struct MimicDB {
    storage: salsa::Storage<Self>,
}

impl Database for MimicDB {}

impl ParallelDatabase for MimicDB {
    fn snapshot(&self) -> Snapshot<Self> {
        Snapshot::new(MimicDB {
            storage: self.storage.snapshot(),
        })
    }
}

#[test]
fn manifest_ast_works() {
    let db = MimicDB::default();
    let toolchain = db.lang_dev_toolchain();
    let package_path_menu = db.package_path_menu(toolchain).as_ref().unwrap();
    expect_file!["../tests/package_core_manifest_ast.txt"].assert_eq(&format!(
        "{:#?}",
        db.package_manifest_ast(package_path_menu.core())
            .as_ref()
            .unwrap()
    ));
    expect_file!["../tests/package_std_manifest_ast.txt"].assert_eq(&format!(
        "{:#?}",
        db.package_manifest_ast(package_path_menu.std())
            .as_ref()
            .unwrap()
    ));
}
