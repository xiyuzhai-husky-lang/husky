pub(crate) use husky_ast::test_utils::*;

use crate::{db::*, *};
use husky_ast::AstJar;
use husky_corgi_config::CorgiConfigJar;
use husky_corgi_config_ast::CorgiConfigAstJar;
use husky_coword::CowordJar;
use husky_declarative_signature::DeclarativeSignatureJar;
use husky_declarative_term::DeclarativeTermJar;

use husky_entity_path::EntityPathJar;
use husky_entity_syn_tree::{helpers::paths::module_item_paths, EntitySynTreeJar};
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
    CowordJar,
    VfsJar,
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
    SynDefnJar,
    SynDeclJar,
    TermPreludeJar,
    DeclarativeTermJar,
    DeclarativeSignatureJar,
    husky_declarative_ty::db::DeclarativeTypeJar,
    EtherealTermJar,
    EtherealSignatureJar,
    FluffyTermJar,
    SemaExprJar,
    husky_hir_ty::db::HirTypeJar,
    husky_hir_eager_expr::db::HirEagerExprJar,
    husky_hir_lazy_expr::db::HirLazyExprJar,
    husky_hir_expr::db::HirExprJar,
    husky_hir_decl::db::HirDeclJar,
    HirDefnJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}

fn module_hir_defns(db: &DB, module_path: ModulePath) -> Vec<ItemHirDefn> {
    module_item_paths(db, module_path)
        .iter()
        .filter_map(|path| path.hir_defn(db))
        .collect()
}

#[test]
fn module_hir_defns_works() {
    DB::default()
        .ast_expect_test_debug_with_db(module_hir_defns, &AstTestConfig::new("module_hir_defns"));
}
