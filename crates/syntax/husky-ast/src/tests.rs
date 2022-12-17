use crate::*;
use expect_test::expect_file;
use husky_absolute_path::AbsolutePath;
use husky_entity_path::{EntityPathData, EntityPathDb, EntityPathJar};
use husky_package_path::{CrateKind, PackagePathData, PackagePathDb, PackagePathJar};
use husky_token::TokenJar;
use husky_toolchain::ToolchainJar;
use husky_vfs::*;
use husky_vfs_test_utils::VfsTestSupport;
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
    AstJar
)]
#[derive(Default)]
struct DB {
    storage: salsa::Storage<Self>,
    source_path_config: VfsConfigMimic,
}

impl HasVfsConfig for DB {
    fn vfs_config(&self) -> &VfsConfig {
        &self.source_path_config
    }
}

impl Database for DB {}

impl ParallelDatabase for DB {
    fn snapshot(&self) -> Snapshot<Self> {
        Snapshot::new(DB {
            storage: self.storage.snapshot(),
            source_path_config: self.source_path_config.clone(),
        })
    }
}

#[test]
fn ast_sheet_works() {
    DB::run_module_expect_tests("ast_sheet", AstDb::ast_sheet);
}

#[test]
fn ast_range_sheet_works() {
    DB::run_module_expect_tests("ast_range_sheet", AstDb::ast_range_sheet);
}
