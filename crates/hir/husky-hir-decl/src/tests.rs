pub(crate) use husky_ast::test_utils::*;
use husky_vfs::ModulePath;

use crate::{jar::*, *};
use husky_corgi_config::CorgiConfigJar;
use husky_corgi_config_ast::CorgiConfigAstJar;
use husky_coword::CowordJar;
use husky_dec_signature::DecSignatureJar;
use husky_dec_term::DecTermJar;
use husky_entity_tree::{helpers::paths::module_item_paths, EntityTreeJar};
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
    husky_vfs::jar::VfsJar,
    husky_entity_path::jar::EntityPathJar,
    husky_text::jar::TextJar,
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
    husky_hir_lazy_expr::jar::HirLazyExprJar,
    husky_hir_expr::jar::HirExprJar,
    HirDeclJar
)]
pub(crate) struct DB;

fn module_hir_decls(db: &::salsa::Db, module_path: ModulePath) -> Vec<HirDecl> {
    module_item_paths(db, module_path)
        .iter()
        .filter_map(|path| path.hir_decl(db))
        .collect()
}

#[test]
fn module_hir_decls_works() {
    DB::ast_expect_test_debug_with_db(
        module_hir_decls,
        &AstTestConfig::new(
            "module_hir_decls",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::HIR,
        ),
    );
}
