use crate::*;
use expect_test::expect_file;
use husky_entity_path::EntityPathJar;

use husky_toml_token::TomlTokenJar;
use husky_vfs::*;
use husky_word::WordJar;
use salsa::{Database, ParallelDatabase, Snapshot};

#[salsa::db(WordJar, VfsJar, TomlTokenJar, TomlAstJar)]
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
    todo!("change it use vfs test utils");
    let db = MimicDB::default();
    let toolchain = db.lang_dev_toolchain();
    let path_menu = db.path_menu(toolchain).as_ref().unwrap();
    expect_file!["../tests/package_core_manifest_ast.txt"].assert_eq(&format!(
        "{:#?}",
        db.package_manifest_ast(path_menu.core_package())
            .as_ref()
            .unwrap()
    ));
    expect_file!["../tests/package_std_manifest_ast.txt"].assert_eq(&format!(
        "{:#?}",
        db.package_manifest_ast(path_menu.std_package())
            .as_ref()
            .unwrap()
    ));
}
