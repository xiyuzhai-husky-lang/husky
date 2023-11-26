pub(crate) use husky_ast::test_utils::*;

use crate::*;
use husky_ast::AstJar;
use husky_corgi_config::CorgiConfigJar;
use husky_corgi_config_ast::CorgiConfigAstJar;
use husky_coword::CowordJar;
use husky_declarative_signature::DeclarativeSignatureJar;
use husky_declarative_term::DeclarativeTermJar;

use husky_entity_path::EntityPathJar;
use husky_entity_syn_tree::EntitySynTreeJar;
use husky_ethereal_signature::EtherealSignatureJar;
use husky_ethereal_term::EtherealTermJar;
use husky_fluffy_term::FluffyTermJar;
use husky_manifest::ManifestJar;
use husky_manifest_ast::ManifestAstJar;
use husky_sema_expr::SemaExprJar;
use husky_syn_decl::SynDeclJar;
use husky_syn_defn::SynDefnJar;
use husky_syn_expr::SynExprJar;
use husky_term_prelude::TermPreludeJar;
use husky_token::TokenJar;
use husky_toml_ast::TomlAstJar;
use husky_toml_token::TomlTokenJar;

#[salsa::db(
    VfsJar,
    CowordJar,
    husky_token_data::db::TokenDataJar,
    TokenJar,
    EntityPathJar,
    TomlTokenJar,
    TomlAstJar,
    ManifestAstJar,
    CorgiConfigJar,
    CorgiConfigAstJar,
    ManifestJar,
    AstJar,
    EntitySynTreeJar,
    SynDeclJar,
    SynDefnJar,
    SynExprJar,
    TermPreludeJar,
    DeclarativeTermJar,
    DeclarativeSignatureJar,
    husky_declarative_ty::db::DeclarativeTypeJar,
    EtherealTermJar,
    EtherealSignatureJar,
    FluffyTermJar,
    SemaExprJar,
    TokenInfoJar
)]
#[derive(Default)]
pub(crate) struct DB;

#[test]
fn token_infer_sheet_works() {
    DB::default().ast_expect_test_debug_with_db(
        TokenInfoDb::token_info_sheet,
        &AstTestConfig::new("token_infer_sheet"),
    )
}
