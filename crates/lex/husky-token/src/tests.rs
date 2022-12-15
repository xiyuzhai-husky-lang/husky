use crate::*;
use expect_test::expect_file;
use husky_entity_path::{EntityPathDb, EntityPathJar};
use husky_expect_test_utils::*;
use husky_package_path::{PackagePathDb, PackagePathJar};
use husky_print_utils::p;
use husky_toolchain::ToolchainJar;
use husky_vfs::{HasVfsConfig, VfsConfig, VfsConfigMimic, VfsJar};
use husky_word::{WordDb, WordJar};
use salsa::{Database, ParallelDatabase, Snapshot, Storage};
use std::{borrow::Cow, sync::Arc};

#[salsa::db(WordJar, ToolchainJar, PackagePathJar, TokenJar, VfsJar, EntityPathJar)]
#[derive(Default)]
struct MimicDB {
    storage: Storage<Self>,
    source_path_config: VfsConfigMimic,
}

impl HasVfsConfig for MimicDB {
    fn vfs_config(&self) -> &VfsConfig {
        &self.source_path_config
    }
}

impl Database for MimicDB {}

#[test]
fn tokenize_works() {
    expect_test_husky_to_rust("batch", &tokenize_debug);

    fn tokenize_debug(text: &str) -> String {
        format!("{:#?}", MimicDB::default().tokenize(text))
    }
}

#[test]
fn tokenize_library() {
    let db = MimicDB::default();
    let package_path_menu = db.package_path_menu();
    let entity_path_menu = db.entity_path_menu();

    macro_rules! t {
        ($($module:ident),*) => {
            $(
                expect_file![format!("../tests/single/{}_token_sheet.txt", stringify!($module))]
                    .assert_eq(&format!("{:#?}", db.token_sheet(entity_path_menu.$module())))
            );*
        }
    }

    t!(core, core_basic, core_num, std);
}
