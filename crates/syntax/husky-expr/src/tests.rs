use crate::*;
use husky_entity_path::EntityPathJar;
use husky_entity_tree::EntityTreeJar;
use husky_expect_test_utils::expect_test_husky_to_rust;
use husky_package_path::PackagePathJar;
use husky_source_path::SourcePathJar;
use husky_symbol_syntax::{SymbolContext, SymbolDb, SymbolJar, SymbolSheet};
use husky_token_sheet::TokenTextJar;
use husky_tokenize::TokenizeDb;
use husky_toolchain::ToolchainJar;
use husky_vfs::VfsJar;
use husky_word::WordJar;
use place::SingleAssignPlace;
use salsa::{Database, ParallelDatabase};

#[test]
fn it_works() {
    expect_test_husky_to_rust("", &test_parse_exprs_debug);

    fn test_parse_exprs_debug(text: &str) -> String {
        let db = MimicDB::default();
        let tokens = db.tokenize_line(text);
        let mut arena = ExprArena::new();
        // ad hoc; todo: preludes
        let mut symbols = db.new_symbol_ctx();
        parse_expr(&db, &tokens, &mut symbols, &mut arena);
        format!("{:#?}", arena)
    }
}

#[salsa::db(
    WordJar,
    VfsJar,
    SourcePathJar,
    ToolchainJar,
    PackagePathJar,
    EntityPathJar,
    TextJar,
    TokenTextJar,
    SymbolJar,
    EntityTreeJar
)]
#[derive(Default)]
struct MimicDB {
    storage: salsa::Storage<Self>,
}

impl Database for MimicDB {}

impl ParallelDatabase for MimicDB {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        todo!()
    }
}
