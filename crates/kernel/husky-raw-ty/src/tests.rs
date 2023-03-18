pub(crate) use husky_ast::test_utils::*;

use crate::*;
use husky_ast::AstJar;
use husky_corgi_config::CorgiConfigJar;
use husky_corgi_config_ast::CorgiConfigAstJar;
use husky_decl::DeclJar;
use husky_entity_path::EntityPathJar;
use husky_entity_tree::{EntityTreeDb, EntityTreeJar};
use husky_expr::ExprJar;
use husky_manifest::ManifestJar;
use husky_manifest_ast::ManifestAstJar;
use husky_raw_term::RawTermJar;
use husky_signature::SignatureJar;
use husky_term_prelude::TermPreludeJar;
use husky_token::TokenJar;
use husky_toml_ast::TomlAstJar;
use husky_toml_token::TomlTokenJar;

use husky_word::WordJar;

#[salsa::db(
    WordJar,
    VfsJar,
    EntityPathJar,
    TokenJar,
    AstJar,
    EntityTreeJar,
    TomlTokenJar,
    TomlAstJar,
    ManifestAstJar,
    CorgiConfigJar,
    CorgiConfigAstJar,
    ManifestJar,
    ExprJar,
    DeclJar,
    TermPreludeJar,
    RawTermJar,
    SignatureJar,
    RawTypeJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}

fn entity_raw_tys(db: &DB, module_path: ModulePath) -> Vec<(EntityPath, RawTypeResult<RawTerm>)> {
    let Ok(entity_tree_sheet) = db.entity_tree_sheet(module_path) else {
        return vec![]
    };
    entity_tree_sheet
        .module_item_path_iter(db)
        .map(|path| {
            (
                path.into(),
                entity_path_raw_ty(db, TypePathDisambiguation::Ontology, path.into()),
            )
        })
        .collect()
}

#[test]
fn entity_raw_tys_works() {
    DB::default().ast_expect_test_debug_with_db("entity_raw_tys", entity_raw_tys)
}
