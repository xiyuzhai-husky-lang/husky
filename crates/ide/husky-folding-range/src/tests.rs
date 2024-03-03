pub(crate) use husky_ast::test_utils::*;

use crate::*;
use husky_coword::CowordJar;
use husky_term_prelude::TermPreludeJar;
use husky_token::TokenJar;

#[salsa::db(
    CowordJar,
    VfsJar,
    husky_entity_path::jar::EntityPathJar,
    TermPreludeJar,
    husky_token_data::jar::TokenDataJar,
    TokenJar,
    husky_ast::jar::AstJar,
    FoldingRangeJar
)]
#[derive(Default)]
pub struct DB;

#[test]
fn folding_ranges_works() {
    DB::ast_expect_test_debug(
        |db, module_path| db.folding_ranges(module_path),
        &AstTestConfig::new(
            "folding_ranges",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::IDE,
        ),
    );
}
