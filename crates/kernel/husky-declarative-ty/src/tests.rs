pub(crate) use husky_ast::test_utils::*;

use crate::*;
use husky_ast::AstJar;
use husky_corgi_config::CorgiConfigJar;
use husky_corgi_config_ast::CorgiConfigAstJar;
use husky_decl::DeclJar;
use husky_declarative_signature::DeclarativeSignatureJar;
use husky_declarative_term::DeclarativeTermJar;
use husky_decr::DecrJar;
use husky_entity_path::EntityPathJar;
use husky_entity_tree::{EntityTreeDb, EntityTreeJar};
use husky_expr::ExprJar;
use husky_manifest::ManifestJar;
use husky_manifest_ast::ManifestAstJar;
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
    DecrJar,
    TermPreludeJar,
    DeclarativeTermJar,
    DeclarativeSignatureJar,
    DeclarativeTypeJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}

fn major_entity_declarative_tys(
    db: &DB,
    module_path: ModulePath,
) -> Vec<(EntityPath, DeclarativeTypeResult<DeclarativeTerm>)> {
    let Ok(entity_tree_sheet) = db.entity_tree_sheet(module_path) else {
        return vec![]
    };
    entity_tree_sheet
        .major_entity_paths(db)
        .map(|path| {
            (
                path.into(),
                entity_path_declarative_ty(
                    db,
                    TypePathDisambiguation::OntologyConstructor,
                    path.into(),
                ),
            )
        })
        .collect()
}

#[test]
fn entity_declarative_tys_works() {
    DB::default()
        .ast_expect_test_debug_with_db("major_entity_declarative_tys", major_entity_declarative_tys)
}
