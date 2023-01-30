use husky_decl::{DeclDb, DeclJar};
use husky_print_utils::p;
use husky_signature::SignatureJar;
use husky_term::TermJar;
pub(crate) use husky_vfs::VfsTestUtils;

use crate::*;
use husky_ast::AstJar;
use husky_entity_path::{EntityPathJar, ModuleItemPath, TypePath};
use husky_entity_tree::{EntityTreeDb, EntityTreeJar};
use husky_expr::ExprJar;
use husky_manifest::ManifestJar;
use husky_token::TokenJar;
use husky_vfs::*;
use husky_word::WordJar;

#[salsa::db(
    WordJar,
    VfsJar,
    EntityPathJar,
    TokenJar,
    AstJar,
    EntityTreeJar,
    ManifestJar,
    ExprJar,
    DeclJar,
    TermJar,
    SignatureJar,
    TypeJar,
    DefnJar,
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
        .filter_map(|decl| Some(db.expr_ty_region(decl.ok()?.expr_region(db))))
        .collect()
}

#[test]
fn decl_expr_ty_sheets_works() {
    DB::default().vfs_expect_test_debug_with_db("decl_expr_ty_regions", decl_expr_ty_regions)
}

fn defn_expr_ty_regions(db: &DB, module_path: ModulePath) -> Vec<&ExprTypeRegion> {
    let Ok(defn_sheet) = db.defn_sheet(module_path)
        else { return vec![] };
    defn_sheet
        .defns()
        .filter_map(|defn| Some(db.expr_ty_region(defn.expr_region(db)?)))
        .collect()
}

#[test]
fn defn_expr_ty_sheets_works() {
    DB::default().vfs_expect_test_debug_with_db("defn_expr_ty_regions", defn_expr_ty_regions)
}
