pub(crate) use husky_vfs::VfsTestUtils;

use crate::*;
use husky_ast::AstJar;
use husky_decl::DeclJar;
use husky_entity_path::{EntityPathJar, ModuleItemPath, TypePath};
use husky_entity_tree::{EntityTreeDb, EntityTreeJar};
use husky_expr::ExprJar;
use husky_manifest::ManifestJar;
use husky_print_utils::p;
use husky_raw_term::RawTermJar;
use husky_signature::SignatureJar;
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
    RawTermJar,
    SignatureJar,
    RawTypeJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}

fn entity_raw_tys(db: &DB, module_path: ModulePath) -> Vec<(EntityPath, RawTypeResult<RawTerm>)> {
    let Ok(entity_tree_sheet) = db.entity_tree_sheet(module_path) else {
        return vec![]
    };
    entity_tree_sheet
        .module_item_path_iter(db)
        .map(|path| {
            (
                path.into(),
                entity_path_raw_ty(
                    db,
                    TypePathDisambiguation::TypeItselfOrTemplate,
                    path.into(),
                ),
            )
        })
        .collect()
}

#[test]
fn entity_raw_tys_works() {
    DB::default().vfs_expect_test_debug_with_db("entity_raw_tys", entity_raw_tys)
}
