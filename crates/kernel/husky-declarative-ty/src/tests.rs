pub(crate) use husky_ast::test_utils::*;

use crate::*;
use husky_ast::AstJar;
use husky_corgi_config::CorgiConfigJar;
use husky_corgi_config_ast::CorgiConfigAstJar;
use husky_coword::CowordJar;
use husky_declarative_signature::DeclarativeSignatureJar;
use husky_declarative_term::DeclarativeTermJar;
use husky_entity_path::EntityPathJar;
use husky_entity_syn_tree::{EntitySynTreeDb, EntitySynTreeJar};
use husky_manifest::ManifestJar;
use husky_manifest_ast::ManifestAstJar;
use husky_syn_decl::SynDeclJar;
use husky_syn_expr::SynExprJar;
use husky_term_prelude::TermPreludeJar;
use husky_token::TokenJar;
use husky_toml_ast::TomlAstJar;
use husky_toml_token::TomlTokenJar;
use husky_vfs::ModulePath;

#[salsa::db(
    CowordJar,
    husky_vfs::db::VfsJar,
    EntityPathJar,
    husky_token_data::db::TokenDataJar,
    TokenJar,
    AstJar,
    EntitySynTreeJar,
    TomlTokenJar,
    TomlAstJar,
    ManifestAstJar,
    CorgiConfigJar,
    CorgiConfigAstJar,
    ManifestJar,
    SynExprJar,
    SynDeclJar,
    TermPreludeJar,
    DeclarativeTermJar,
    DeclarativeSignatureJar,
    DeclarativeTypeJar
)]
#[derive(Default)]
pub(crate) struct DB;

fn major_item_declarative_tys(
    db: &::salsa::Db,
    module_path: ModulePath,
) -> Vec<(ItemPath, DeclarativeTypeResult<DeclarativeTerm>)> {
    db.item_syn_tree_sheet(module_path)
        .major_paths(db)
        .map(|path| {
            (
                path.into(),
                item_path_declarative_ty(
                    db,
                    TypePathDisambiguation::OntologyConstructor,
                    path.into(),
                ),
            )
        })
        .collect()
}

#[test]
fn item_declarative_tys_works() {
    DB::default().ast_expect_test_debug_with_db(
        major_item_declarative_tys,
        &AstTestConfig::new("major_item_declarative_tys"),
    )
}
