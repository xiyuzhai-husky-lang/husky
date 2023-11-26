pub(crate) use husky_ast::test_utils::*;

use crate::*;
use husky_ast::AstJar;
use husky_coword::CowordJar;
use husky_entity_path::EntityPathJar;
use husky_term_prelude::TermPreludeJar;
use husky_token::TokenJar;

#[salsa::test_db(
    CowordJar,
    VfsJar,
    EntityPathJar,
    TermPreludeJar,
    husky_token_data::db::TokenDataJar,
    TokenJar,
    AstJar,
    FoldingRangeJar
)]
#[derive(Default)]
pub struct DB;

#[test]
fn folding_ranges_works() {
    DB::default().ast_expect_test_debug(
        |db, module_path| db.folding_ranges(module_path),
        &AstTestConfig::new("folding_ranges"),
    );
}
