pub(crate) use husky_ast::test_utils::*;

use crate::*;
use husky_corgi_config::CorgiConfigJar;
use husky_corgi_config_ast::CorgiConfigAstJar;
use husky_coword::CowordJar;
use husky_dec_signature::DecSignatureJar;

use husky_entity_tree::EntityTreeJar;
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
use husky_token::TokenJar;
use husky_toml_ast::TomlAstJar;
use husky_toml_token::TomlTokenJar;

#[salsa::db(
    VfsJar,
    CowordJar,
    husky_token_data::db::TokenDataJar,
    TokenJar,
    husky_entity_path::jar::EntityPathJar,
    TomlTokenJar,
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
    DiagnosticsJar,
    TermPreludeJar,
    husky_dec_term::jar::DecTermJar,
    husky_dec_ty::jar::DeclarativeTypeJar,
    EthTermJar,
    EtherealSignatureJar,
    DecSignatureJar,
    FlyTermJar,
    SemaExprJar
)]
#[derive(Default)]
pub(crate) struct DB;
