pub(crate) use husky_ast::test_utils::*;

use crate::*;
use husky_corgi_config::CorgiConfigJar;
use husky_corgi_config_ast::CorgiConfigAstJar;
use husky_coword::CowordJar;
use husky_dec_signature::DecSignatureJar;
use husky_eth_signature::EtherealSignatureJar;
use husky_eth_term::EthTermJar;
use husky_fly_term::FlyTermJar;
use husky_manifest::ManifestJar;
use husky_manifest_ast::ManifestAstJar;
use husky_sema_expr::SemaExprJar;
use husky_syn_decl::SynDeclJar;
use husky_syn_defn::SynDefnJar;
use husky_syn_expr::SynExprJar;
use husky_term_prelude::TermPreludeJar;
use husky_toml_ast::TomlAstJar;
use husky_toml_token::jar::TomlTokenJar;

#[salsa::db(
    VfsJar,
    CowordJar,
    husky_token_data::jar::TokenDataJar,
    TokenJar,
    husky_entity_path::jar::EntityPathJar,
    husky_toml_token::jar::TomlTokenJar,
    TomlAstJar,
    ManifestAstJar,
    CorgiConfigJar,
    CorgiConfigAstJar,
    ManifestJar,
    husky_ast::jar::AstJar,
    EntityTreeJar,
    SynDeclJar,
    SynDefnJar,
    SynExprJar,
    TermPreludeJar,
    husky_dec_term::jar::DecTermJar,
    DecSignatureJar,
    husky_dec_ty::jar::DeclarativeTypeJar,
    EthTermJar,
    EtherealSignatureJar,
    FlyTermJar,
    SemaExprJar,
    husky_sema_place_contract::jar::SemaPlaceContractJar,
    TokenInfoJar
)]
#[derive(Default)]
pub(crate) struct DB;

#[test]
fn token_infer_sheet_works() {
    DB::ast_expect_test_debug_with_db(
        TokenInfoDb::token_info_sheet,
        &AstTestConfig::new(
            "token_infer_sheet",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::IDE,
        ),
    )
}
