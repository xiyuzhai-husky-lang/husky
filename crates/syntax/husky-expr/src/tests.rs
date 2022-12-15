use crate::*;
use husky_absolute_path::AbsolutePathJar;
use husky_entity_path::EntityPathJar;
use husky_entity_tree::EntityTreeJar;
use husky_expect_test_utils::expect_test_husky_to_rust;
use husky_package_path::PackagePathJar;
use husky_source_path::{HasSourcePathConfig, SourcePathConfig, SourcePathJar};
use husky_symbol_syntax::{SymbolContext, SymbolDb, SymbolJar, SymbolSheet};
use husky_token::TokenJar;
use husky_token::{TokenDb, Tokenize};
use husky_toolchain::ToolchainJar;
use husky_vfs::VfsJar;
use husky_word::WordJar;
use place::SingleAssignPlace;
use salsa::{Database, ParallelDatabase};

#[salsa::db(
    WordJar,
    VfsJar,
    AbsolutePathJar,
    SourcePathJar,
    ToolchainJar,
    PackagePathJar,
    EntityPathJar,
    TokenJar,
    SymbolJar,
    EntityTreeJar
)]
#[derive(Default)]
struct MimicDB {
    storage: salsa::Storage<Self>,
}

impl HasSourcePathConfig for MimicDB {
    fn source_path_config(&self) -> &SourcePathConfig {
        todo!()
    }
}

impl Database for MimicDB {}

#[test]
fn it_works() {
    expect_test_husky_to_rust("", &test_parse_exprs_debug);

    fn test_parse_exprs_debug(text: &str) -> String {
        let db = MimicDB::default();
        let tokens = db.tokenize(text);
        let mut arena = ExprArena::default();
        // ad hoc; todo: preludes
        let mut symbols = db.new_symbol_ctx();
        parse_expr(&db, &tokens, &mut symbols, &mut arena);
        format!("{:#?}", arena)
    }
}
