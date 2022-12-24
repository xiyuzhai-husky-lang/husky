use crate::*;
use husky_ast::AstJar;
use husky_entity_path::EntityPathJar;
use husky_entity_tree::EntityTreeJar;
use husky_manifest::ManifestJar;
use husky_token::{TokenIter, TokenJar};
use husky_vfs::*;
use husky_word::WordJar;
use salsa::Database;

#[salsa::db(
    WordJar,
    VfsJar,
    EntityPathJar,
    TokenJar,
    AstJar,
    EntityTreeJar,
    ManifestJar
)]
#[derive(Default)]
struct DB {
    storage: salsa::Storage<Self>,
}

impl Database for DB {}

// #[test]
// fn parse_exprs_works() {
//     expect_test_husky_to_rust("", &test_parse_exprs_debug);

//     fn test_parse_exprs_debug(text: &str) -> String {
//         let db = MimicDB::default();
//         let tokens = db.tokenize(text);
//         let mut arena = ExprArena::default();
//         // ad hoc; todo: preludes
//         let mut symbols = db.new_symbol_ctx();
//         parse_expr(&db, TokenIter::new(0, &tokens), &mut symbols, &mut arena);
//         format!("{:#?}", arena)
//     }
// }
