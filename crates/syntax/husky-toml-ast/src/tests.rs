use crate::*;
use expect_test::expect_file;
use husky_entity_path::EntityPathJar;
use husky_package_path::{PackagePathDb, PackagePathJar};
use husky_print_utils::p;
use husky_toml_token::TomlTokenJar;
use husky_toolchain::ToolchainJar;
use husky_vfs::*;
use husky_word::{WordDb, WordJar};
use salsa::{Database, ParallelDatabase, Snapshot};
use std::{borrow::Cow, sync::Arc};

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
    source_path_config: VfsConfigMimic,
}

impl HasVfsConfig for MimicDB {
    fn vfs_config(&self) -> &VfsConfig {
        &self.source_path_config
    }
}

impl Database for MimicDB {}

impl ParallelDatabase for MimicDB {
    fn snapshot(&self) -> Snapshot<Self> {
        Snapshot::new(MimicDB {
            storage: self.storage.snapshot(),
            source_path_config: self.source_path_config.clone(),
        })
    }
}

#[test]
fn manifest_ast_works() {
    let db = MimicDB::default();
    let package_path_menu = db.package_path_menu();
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
