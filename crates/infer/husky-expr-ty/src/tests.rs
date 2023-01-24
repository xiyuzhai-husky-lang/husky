use husky_decl::DeclJar;
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
    ExprTypeJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}

fn expr_ty_sheets(db: &DB, module_path: ModulePath) -> Vec<ExprTypeSheet> {
    todo!()
}

#[test]
fn expr_ty_sheets_works() {
    DB::default().vfs_expect_test_debug_with_db("expr_ty_sheets", expr_ty_sheets)
}
