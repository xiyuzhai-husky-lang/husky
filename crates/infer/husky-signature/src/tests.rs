pub(crate) use husky_vfs::VfsTestUtils;

use crate::*;
use husky_ast::AstJar;
use husky_entity_path::EntityPathJar;
use husky_entity_tree::EntityTreeJar;
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
    SignatureJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}

fn module_signatures(db: &DB, module_path: ModulePath) -> Vec<Signature> {
    let decl_sheet = db.module_decl_sheet(module_path).unwrap();
    decl_sheet
        .decls()
        .iter()
        .filter_map(|decl| match decl {
            Ok(decl) => Some(db.signature(*decl)),
            Err(_) => None,
        })
        .collect()
}

#[test]
fn module_signatures_works() {
    DB::default().vfs_expect_test_debug_with_db("signature", module_signatures)
}
