pub(crate) use husky_ast::test_utils::*;

use crate::*;
use husky_corgi_config::jar::CorgiConfigJar;
use husky_corgi_config_ast::CorgiConfigAstJar;
use husky_coword::jar::CowordJar;
use husky_dec_signature::jar::DecSignatureJar;

use husky_entity_tree::EntityTreeJar;
use husky_eth_signature::jar::EthSignatureJar;
use husky_eth_term::jar::EthTermJar;
use husky_fly_term::jar::FlyTermJar;
use husky_manifest::jar::ManifestJar;
use husky_manifest_ast::jar::ManifestAstJar;
use husky_sem_expr::SemExprJar;
use husky_syn_decl::SynDeclJar;
use husky_syn_defn::jar::SynDefnJar;
use husky_syn_expr::jar::SynExprJar;
use husky_term_prelude::jar::TermPreludeJar;
use husky_token::TokenJar;
use husky_toml_ast::TomlAstJar;

#[salsa::db(
    VfsJar,
    CowordJar,
    husky_text::jar::TextJar,
    husky_token_data::jar::TokenDataJar,
    TokenJar,
    TokenInfoJar,
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
    EthSignatureJar,
    FlyTermJar,
    SemExprJar,
    husky_sem_place_contract::jar::SemPlaceContractJar,
    SemanticTokenJar
)]
#[derive(Default)]
pub(crate) struct DB;

#[test]
fn semantic_tokens_works() {
    DB::ast_expect_test_debug(
        |db, module_path| SemanticTokenDb::semantic_tokens_ext(db, module_path, None),
        &AstTestConfig::new(
            "semantic_tokens",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::IDE,
        ),
    )
}
