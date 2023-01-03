use crate::*;
use husky_ast::AstJar;
use husky_decl::DeclJar;
use husky_defn::DefnJar;
use husky_entity_path::EntityPathJar;
use husky_entity_tree::{EntityTreeJar, EntityTreeResult};
use husky_expr::ExprJar;
use husky_manifest::ManifestJar;
use husky_token::{TokenDb, TokenIdx, TokenJar};
use husky_token_info::TokenInfoJar;
use husky_word::WordJar;

#[salsa::db(
    VfsJar,
    WordJar,
    TokenJar,
    TokenInfoJar,
    EntityPathJar,
    ManifestJar,
    AstJar,
    EntityTreeJar,
    DeclJar,
    DefnJar,
    ExprJar,
    HoverJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}

#[test]
fn hover_result_works() {
    const N: usize = 20;
    DB::expect_test_probable_modules_debug(
        "hover_result",
        |db, module_path| -> EntityTreeResult<Vec<(TokenIdx, Option<HoverResult>)>> {
            let token_sheet = db.token_sheet(module_path)?;
            let len = token_sheet.tokens().len();
            let step = (len / N).max(1);
            let mut hover_results = vec![];
            for token_idx in token_sheet.token_index_iter() {
                // only push some of them, but all of them have to be computed
                let hover_result = calc_hover_result(db, module_path, token_idx)?;
                if token_idx.raw() % step == 0 {
                    hover_results.push((token_idx, hover_result))
                }
            }
            Ok(hover_results)
        },
    )
}
