pub use husky_ast::test_utils::*;

#[salsa::db(
    husky_coword::CowordJar,
    husky_vfs::VfsJar,
    husky_entity_path::jar::EntityPathJar,
    husky_text::jar::TextJar,
    husky_token_data::jar::TokenDataJar,
    husky_token::TokenJar,
    husky_ast::jar::AstJar,
    husky_entity_tree::EntityTreeJar,
    husky_toml_token::jar::TomlTokenJar,
    husky_toml_ast::TomlAstJar,
    husky_manifest_ast::ManifestAstJar,
    husky_corgi_config::CorgiConfigJar,
    husky_corgi_config_ast::CorgiConfigAstJar,
    husky_manifest::ManifestJar,
    husky_syn_expr::jar::SynExprJar,
    husky_syn_defn::SynDefnJar,
    husky_syn_decl::SynDeclJar,
    husky_term_prelude::TermPreludeJar,
    husky_dec_term::jar::DecTermJar,
    husky_dec_signature::DecSignatureJar,
    husky_dec_ty::jar::DeclarativeTypeJar,
    husky_eth_term::EthTermJar,
    husky_eth_signature::EtherealSignatureJar,
    husky_fly_term::FlyTermJar,
    husky_sem_expr::SemExprJar,
    crate::Jar
)]
#[derive(Default)]
pub(crate) struct DB;
