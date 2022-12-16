use crate::*;
use expect_test::expect_file;
use husky_absolute_path::AbsolutePath;
use husky_ast::AstJar;
use husky_entity_path::{CratePathKind, EntityPathData, EntityPathDb, EntityPathJar};
use husky_package_path::{PackagePathData, PackagePathDb, PackagePathJar};
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
fn folding_ranges_works() {
    DB::run_module_expect_tests("ast_sheet", FoldingRangeDb::folding_ranges);
}
