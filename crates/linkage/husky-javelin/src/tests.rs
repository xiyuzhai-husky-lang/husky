pub(crate) use husky_ast::test_utils::*;

use husky_corgi_config::CorgiConfigJar;
use husky_corgi_config_ast::CorgiConfigAstJar;
use husky_coword::CowordJar;
use husky_dec_signature::DecSignatureJar;
use husky_dec_term::DecTermJar;
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
    CowordJar,
    husky_vfs::VfsJar,
    husky_entity_path::jar::EntityPathJar,
    husky_token_data::db::TokenDataJar,
    TokenJar,
    husky_ast::jar::AstJar,
    EntityTreeJar,
    TomlTokenJar,
    TomlAstJar,
    ManifestAstJar,
    CorgiConfigJar,
    CorgiConfigAstJar,
    ManifestJar,
    SynExprJar,
    SynDefnJar,
    SynDeclJar,
    TermPreludeJar,
    DecTermJar,
    DecSignatureJar,
    husky_dec_ty::db::DeclarativeTypeJar,
    EthTermJar,
    EtherealSignatureJar,
    FlyTermJar,
    SemaExprJar,
    husky_sema_place_contract::jar::SemaPlaceContractJar,
    husky_hir_ty::db::HirTypeJar,
    husky_hir_eager_expr::jar::HirEagerExprJar,
    husky_hir_lazy_expr::db::HirLazyExprJar,
    husky_hir_expr::db::HirExprJar,
    husky_hir_decl::jar::HirDeclJar,
    husky_hir_defn::jar::HirDefnJar,
    crate::jar::JavelinJar
)]
#[derive(Default)]
pub(crate) struct DB;
