pub(crate) use husky_ast::test_utils::*;

use crate::*;

use husky_ast::AstJar;
use husky_corgi_config::CorgiConfigJar;
use husky_corgi_config_ast::CorgiConfigAstJar;
use husky_coword::CowordJar;
use husky_item_path::EntityPathJar;
use husky_item_tree::EntitySynTreeJar;
use husky_manifest::ManifestJar;
use husky_manifest_ast::ManifestAstJar;
use husky_term_prelude::TermPreludeJar;
use husky_token::TokenJar;
use husky_toml_ast::TomlAstJar;
use husky_toml_token::TomlTokenJar;

#[salsa::db(
    CowordJar,
    VfsJar,
    EntityPathJar,
    TermPreludeJar,
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
    SynDeclJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}

#[test]
fn menu_item_decl_works() {
    let db = DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let item_path_menu = db.item_path_menu(toolchain);
    let i32_ty_path_decl = item_path_menu.i32_ty_path().syn_decl(&db).unwrap();
    salsa::assert_eq_with_db!(db, i32_ty_path_decl.generic_parameters(&db).len(), 0);
}
