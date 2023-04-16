pub(crate) use husky_ast::test_utils::*;

use crate::*;
use husky_ast::AstJar;
use husky_corgi_config::CorgiConfigJar;
use husky_corgi_config_ast::CorgiConfigAstJar;
use husky_decl::DeclJar;
use husky_decr::DecrJar;
use husky_defn::DefnJar;
use husky_entity_path::EntityPathJar;
use husky_entity_tree::EntityTreeJar;
use husky_ethereal_term::EtherealTermJar;
use husky_ethereal_ty::EtherealTypeJar;
use husky_expr::ExprJar;
use husky_expr_ty::ExprTypeJar;
use husky_fluffy_term::FluffyTermJar;
use husky_manifest::ManifestJar;
use husky_manifest_ast::ManifestAstJar;
use husky_raw_term::RawTermJar;
use husky_raw_ty::RawTypeJar;
use husky_signature::SignatureJar;
use husky_term_prelude::TermPreludeJar;
use husky_token::TokenJar;
use husky_toml_ast::TomlAstJar;
use husky_toml_token::TomlTokenJar;
use husky_word::WordJar;

#[salsa::db(
    VfsJar,
    WordJar,
    TokenJar,
    EntityPathJar,
    TomlTokenJar,
    TomlAstJar,
    ManifestAstJar,
    CorgiConfigJar,
    CorgiConfigAstJar,
    ManifestJar,
    AstJar,
    EntityTreeJar,
    DeclJar,
    DecrJar,
    DefnJar,
    ExprJar,
    TermPreludeJar,
    RawTermJar,
    SignatureJar,
    RawTypeJar,
    EtherealTermJar,
    EtherealTypeJar,
    FluffyTermJar,
    ExprTypeJar,
    TokenInfoJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}

#[test]
fn token_infer_sheet_works() {
    DB::default().ast_expect_test_debug_with_db("token_infer_sheet", TokenInfoDb::token_info_sheet)
}
