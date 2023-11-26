pub(crate) use husky_ast::test_utils::*;
pub(crate) use salsa::test_utils::Db;

use crate::*;
use husky_ast::AstJar;
use husky_corgi_config::CorgiConfigJar;
use husky_corgi_config_ast::CorgiConfigAstJar;
use husky_coword::CowordJar;
use husky_declarative_signature::DeclarativeSignatureJar;
use husky_declarative_term::DeclarativeTermJar;
use husky_declarative_ty::DeclarativeTypeJar;
use husky_entity_path::{EntityPathJar, MajorItemPath, TypePath};
use husky_entity_syn_tree::{EntitySynTreeDb, EntitySynTreeJar};
use husky_ethereal_signature::EtherealSignatureJar;
use husky_ethereal_term::EtherealTermJar;
use husky_manifest::ManifestJar;
use husky_manifest_ast::ManifestAstJar;
use husky_print_utils::p;
use husky_syn_decl::{SynDeclDb, SynDeclJar};
use husky_syn_expr::SynExprJar;
use husky_term_prelude::TermPreludeJar;
use husky_token::TokenJar;
use husky_toml_ast::TomlAstJar;
use husky_toml_token::TomlTokenJar;
use husky_vfs::*;

#[salsa::test_db(
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
    SemaExprJar
)]
#[derive(Default)]
pub(crate) struct DB;

fn decl_sema_expr_regions(db: &::salsa::Db, module_path: ModulePath) -> Vec<SemaExprRegion> {
    db.syn_decl_sheet(module_path)
        .decls(db)
        .iter()
        .copied()
        .filter_map(|(_, decl)| Some(db.sema_expr_region(decl.syn_expr_region(db)?)))
        .collect()
}

#[test]
fn decl_sema_expr_sheets_works() {
    DB::default().ast_expect_test_debug_with_db(
        decl_sema_expr_regions,
        &AstTestConfig::new("decl_sema_expr_regions"),
    )
}

fn defn_sema_expr_regions(db: &::salsa::Db, module_path: ModulePath) -> Vec<SemaExprRegion> {
    let Ok(defns) = module_path.defns(db) else {
        return vec![];
    };
    defns
        .iter()
        .copied()
        .filter_map(|defn| Some(db.sema_expr_region(defn.syn_expr_region(db)?)))
        .collect()
}

#[test]
fn defn_sema_expr_regions_works() {
    DB::default().ast_expect_test_debug_with_db(
        defn_sema_expr_regions,
        &AstTestConfig::new("defn_sema_expr_regions"),
    )
}
