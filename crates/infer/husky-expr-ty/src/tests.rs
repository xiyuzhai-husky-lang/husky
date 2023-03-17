pub(crate) use husky_ast::test_utils::*;

use crate::*;
use husky_ast::AstJar;
use husky_decl::{DeclDb, DeclJar};
use husky_entity_path::{EntityPathJar, ModuleItemPath, TypePath};
use husky_entity_tree::{EntityTreeDb, EntityTreeJar};
use husky_expr::ExprJar;
use husky_manifest::ManifestJar;
use husky_manifest_ast::ManifestAstJar;
use husky_print_utils::p;
use husky_raw_term::RawTermJar;
use husky_raw_ty::RawTypeJar;
use husky_signature::SignatureJar;
use husky_term::TermJar;
use husky_term_prelude::TermPreludeJar;
use husky_token::TokenJar;
use husky_toml_ast::TomlAstJar;
use husky_toml_token::TomlTokenJar;
use husky_vfs::*;
use husky_word::WordJar;

#[salsa::db(
    WordJar,
    VfsJar,
    EntityPathJar,
    TokenJar,
    AstJar,
    EntityTreeJar,
    TomlTokenJar,
    TomlAstJar,
    ManifestAstJar,
    ManifestJar,
    ExprJar,
    DefnJar,
    DeclJar,
    TermPreludeJar,
    RawTermJar,
    SignatureJar,
    RawTypeJar,
    TermJar,
    ExprTypeJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}

fn decl_expr_ty_regions(db: &DB, module_path: ModulePath) -> Vec<&ExprTypeRegion> {
    let Ok(decl_sheet) = db.decl_sheet(module_path)
        else { return vec![] };
    decl_sheet
        .decls()
        .iter()
        .filter_map(|(_, decl)| decl.as_ref().ok().copied())
        .map(|decl| db.expr_ty_region(decl.expr_region(db)))
        .collect()
}

#[test]
fn decl_expr_ty_sheets_works() {
    DB::default().ast_expect_test_debug_with_db("decl_expr_ty_regions", decl_expr_ty_regions)
}

fn defn_expr_ty_regions(db: &DB, module_path: ModulePath) -> Vec<&ExprTypeRegion> {
    let Ok(defn_sheet) = db.collect_defns(module_path)
        else { return vec![] };
    defn_sheet
        .defns()
        .filter_map(|(_, defn)| {
            defn.as_ref()
                .ok()
                .copied()
                .map(|defn| Some(db.expr_ty_region(defn.expr_region(db)?)))
                .flatten()
        })
        .collect()
}

#[test]
fn defn_expr_ty_sheets_works() {
    DB::default().ast_expect_test_debug_with_db("defn_expr_ty_regions", defn_expr_ty_regions)
}
