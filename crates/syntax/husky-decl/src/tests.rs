pub(crate) use husky_ast::test_utils::*;

use crate::*;

use husky_ast::AstJar;
use husky_entity_path::EntityPathJar;
use husky_entity_tree::EntityTreeJar;
use husky_manifest::ManifestJar;
use husky_manifest_ast::ManifestAstJar;
use husky_token::TokenJar;
use husky_toml_ast::TomlAstJar;
use husky_toml_token::TomlTokenJar;
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
    DeclJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}

#[test]
fn menu_entity_decl_works() {
    let db = DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let entity_path_menu = db.entity_path_menu(toolchain).unwrap();
    let i32_ty_path_decl = db.ty_decl(entity_path_menu.i32_ty_path()).unwrap();
    salsa::assert_eq_with_db!(db, i32_ty_path_decl.implicit_parameters(&db), Ok(&[]));
    let ref_ty_path_decl = db.ty_decl(entity_path_menu.ref_ty_path()).unwrap();
    assert!(ref_ty_path_decl.implicit_parameters(&db).is_ok());
}
