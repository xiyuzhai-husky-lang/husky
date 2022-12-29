use crate::*;
use husky_ast::AstJar;
use husky_decl::DeclJar;
use husky_defn::DefnJar;
use husky_entity_path::EntityPathJar;
use husky_entity_tree::{EntityTreeJar, EntityTreeResult};
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
    HoverJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}

#[test]
fn hover_result_works() {
    DB::expect_test_probable_modules_debug(
        "hover_result",
        |db, module_path| -> EntityTreeResult<Vec<(TokenIdx, Option<HoverResult>)>> {
            let token_sheet = db.token_sheet(module_path)?;
            token_sheet
                .token_index_iter()
                .map(|token_idx| Ok((token_idx, calc_hover_result(db, module_path, token_idx)?)))
                .collect::<EntityTreeResult<Vec<_>>>()
        },
    )
}
