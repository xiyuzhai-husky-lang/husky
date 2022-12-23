use crate::*;
use husky_ast::AstJar;
use husky_entity_path::EntityPathJar;
use husky_expr::ExprJar;
use husky_manifest::ManifestJar;
use husky_word::WordJar;

#[salsa::db(
    VfsJar,
    WordJar,
    TokenJar,
    TokenInferJar,
    EntityPathJar,
    ManifestJar,
    AstJar,
    EntityTreeJar,
    ExprJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}

#[test]
fn token_infer_sheet_works() {
    DB::expect_test_probable_modules_debug("token_infer_sheet", TokenInferDb::token_infer_sheet)
}
