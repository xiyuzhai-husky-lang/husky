use crate::*;
use expect_test::expect_file;
use husky_absolute_path::AbsolutePath;
use husky_ast::AstJar;
use husky_entity_path::{CratePathKind, EntityPathData, EntityPathDb, EntityPathJar};
use husky_package_path::{PackagePathData, PackagePathDb, PackagePathJar};
use husky_token::TokenJar;
use husky_toolchain::ToolchainJar;
use husky_vfs::*;
use husky_word::{WordDb, WordJar};
use salsa::{Database, DebugWithDb, ParallelDatabase, Snapshot};
use std::{borrow::Cow, sync::Arc};

#[salsa::db(
    WordJar,
    ToolchainJar,
    PackagePathJar,
    EntityPathJar,
    VfsJar,
    TokenJar,
    AstJar,
    FoldingRangeJar
)]
#[derive(Default)]
pub struct DB {
    storage: salsa::Storage<Self>,
    source_path_config: VfsConfigMimic,
}

impl HasVfsConfig for DB {
    fn vfs_config(&self) -> &VfsConfig {
        &self.source_path_config
    }
}

impl salsa::Database for DB {}

#[test]
fn library_folding_ranges_works() {
    let db = DB::default();
    let package_path_menu = db.package_path_menu();
    let entity_path_menu = db.entity_path_menu();

    macro_rules! t {
        ($($module:ident),*) => {
            $(
                expect_file![format!("../tests/library/{}.folding_ranges.txt", stringify!($module))]
                    .assert_eq(&format!("{:#?}", db.folding_ranges(entity_path_menu.$module())))
            );*
        }
    }

    t!(core, core_basic, core_num, std);
}
