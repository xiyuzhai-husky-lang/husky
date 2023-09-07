pub(crate) use husky_ast::test_utils::*;

use crate::{db::*, *};
use husky_ast::AstJar;
use husky_corgi_config::CorgiConfigJar;
use husky_corgi_config_ast::CorgiConfigAstJar;
use husky_coword::CowordJar;
use husky_declarative_signature::DeclarativeSignatureJar;
use husky_declarative_term::DeclarativeTermJar;
use husky_declarative_ty::DeclarativeTypeJar;
use husky_entity_path::{EntityPathJar, MajorItemPath, TypePath};
use husky_entity_syn_tree::{paths::module_item_paths, EntitySynTreeDb, EntitySynTreeJar};
use husky_ethereal_signature::EtherealSignatureJar;
use husky_ethereal_term::EtherealTermJar;
use husky_expr_ty::ExprTypeJar;
use husky_fluffy_term::FluffyTermJar;
use husky_manifest::ManifestJar;
use husky_manifest_ast::ManifestAstJar;
use husky_print_utils::p;
use husky_syn_decl::{SynDeclDb, SynDeclJar};
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
    DeclarativeTypeJar,
    EtherealTermJar,
    EtherealSignatureJar,
    FluffyTermJar,
    ExprTypeJar,
    husky_hir_ty::db::HirTypeJar,
    husky_hir_eager_expr::db::HirEagerExprJar,
    husky_hir_lazy_expr::db::HirLazyExprJar,
    husky_hir_expr::db::HirExprJar,
    HirDeclJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}

fn module_hir_decls(db: &DB, module_path: ModulePath) -> Vec<HirDecl> {
    module_item_paths(db, module_path)
        .as_ref()
        .expect("all modules should be guaranteed to be valid")
        .iter()
        .filter_map(|path| path.hir_decl(db))
        .collect()
}

#[test]
fn module_hir_decls_works() {
    DB::default().ast_expect_test_debug_with_db("module_hir_decls", module_hir_decls);
}
