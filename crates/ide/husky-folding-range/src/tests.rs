use crate::*;
use expect_test::expect_file;
use husky_absolute_path::AbsolutePath;
use husky_ast::AstJar;
use husky_entity_path::{EntityPathData, EntityPathDb, EntityPathJar};
use husky_package_path::{CrateKind, PackagePathData, PackagePathDb, PackagePathJar};
use husky_token::TokenJar;
use husky_toolchain::*;
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
}

impl salsa::Database for DB {}

#[test]
fn folding_ranges_works() {
    DB::expect_test_probable_modules_debug("folding_ranges", FoldingRangeDb::folding_ranges);
}
