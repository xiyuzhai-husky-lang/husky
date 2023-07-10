pub(crate) use husky_ast::test_utils::*;

use crate::*;
use husky_ast::AstJar;
use husky_coword::CowordJar;
use husky_entity_path::EntityPathJar;
use husky_term_prelude::TermPreludeJar;
use husky_token::TokenJar;

#[salsa::db(
    CowordJar,
    VfsJar,
    EntityPathJar,
    TermPreludeJar,
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
    DB::default().ast_expect_test_debug("folding_ranges", FoldingRangeDb::folding_ranges);
}
