pub(crate) use husky_ast::test_utils::*;

use crate::*;
use husky_ast::AstJar;
use husky_corgi_config::CorgiConfigJar;
use husky_corgi_config_ast::CorgiConfigAstJar;
use husky_coword::CowordJar;
use husky_declarative_signature::DeclarativeSignatureJar;
use husky_declarative_term::DeclarativeTermJar;
use husky_declarative_ty::DeclarativeTypeJar;
use husky_entity_path::{EntityPathJar, ModuleItemPath, TypePath};
use husky_entity_tree::{EntitySynTreeDb, EntitySynTreeJar};
use husky_ethereal_signature::EtherealSignatureJar;
use husky_ethereal_term::EtherealTermJar;
use husky_manifest::ManifestJar;
use husky_manifest_ast::ManifestAstJar;
use husky_print_utils::p;
use husky_syn_decl::{DeclDb, SynDeclJar};
use husky_syn_decr::SynDecrJar;
use husky_syn_expr::SynExprJar;
use husky_term_prelude::TermPreludeJar;
use husky_token::TokenJar;
use husky_toml_ast::TomlAstJar;
use husky_toml_token::TomlTokenJar;
use husky_vfs::*;

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
    SynDecrJar,
    SynDeclJar,
    TermPreludeJar,
    DeclarativeTermJar,
    DeclarativeSignatureJar,
    DeclarativeTypeJar,
    EtherealTermJar,
    EtherealSignatureJar,
    FluffyTermJar,
    ExprTypeJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}

fn decl_expr_ty_regions(db: &DB, module_path: ModulePath) -> Vec<&ExprTypeRegion> {
    let Ok(syn_decl_sheet) = db.syn_node_decl_sheet(module_path) else {
        return vec![];
    };
    syn_decl_sheet
        .decls(db)
        .iter()
        .copied()
        .filter_map(|(_, decl)| Some(db.expr_ty_region(decl.expr_region(db)?)))
        .collect()
}

#[test]
fn decl_expr_ty_sheets_works() {
    DB::default().ast_expect_test_debug_with_db("decl_expr_ty_regions", decl_expr_ty_regions)
}

fn defn_expr_ty_regions(db: &DB, module_path: ModulePath) -> Vec<&ExprTypeRegion> {
    let Ok(defns) = module_path.defns(db) else {
        return vec![];
    };
    defns
        .iter()
        .copied()
        .filter_map(|defn| Some(db.expr_ty_region(defn.expr_region(db)?)))
        .collect()
}

#[test]
fn defn_expr_ty_sheets_works() {
    DB::default().ast_expect_test_debug_with_db("defn_expr_ty_regions", defn_expr_ty_regions)
}
