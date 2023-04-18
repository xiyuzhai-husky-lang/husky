pub(crate) use husky_ast::test_utils::*;

use crate::*;
use husky_ast::AstJar;
use husky_corgi_config::CorgiConfigJar;
use husky_corgi_config_ast::CorgiConfigAstJar;
use husky_decr::DecrJar;
use husky_entity_path::EntityPathJar;
use husky_entity_tree::EntityTreeJar;
use husky_expr::ExprJar;
use husky_manifest::ManifestJar;
use husky_manifest_ast::ManifestAstJar;
use husky_term_prelude::TermPreludeJar;
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
    CorgiConfigJar,
    CorgiConfigAstJar,
    ManifestJar,
    ExprJar,
    DeclJar,
    DecrJar,
    TermPreludeJar,
    DeclarativeTermJar,
    DeclarativeSignatureTemplateJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}

fn module_signatures(
    db: &DB,
    module_path: ModulePath,
) -> Vec<DeclarativeSignatureResult<Signature>> {
    let Ok(decl_sheet) = db.decl_sheet(module_path) else {
        return vec![]
    };
    decl_sheet
        .decls()
        .iter()
        .filter_map(|decl| {
            decl.1
                .as_ref()
                .ok()
                .copied()
                .map(|decl| signature_from_decl(db, decl))
        })
        .collect()
}

#[test]
fn module_signatures_works() {
    DB::default().ast_expect_test_debug_with_db("signature", module_signatures)
}

#[test]
fn menu_ty_declarative_signature_templates_works() {
    let db = DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let entity_path_menu = db.entity_path_menu(toolchain);
    let ty_paths = vec![
        entity_path_menu.i16_ty_path(),
        entity_path_menu.i32_ty_path(),
        entity_path_menu.i64_ty_path(),
        entity_path_menu.u8_ty_path(),
        entity_path_menu.u16_ty_path(),
        entity_path_menu.u32_ty_path(),
        entity_path_menu.u64_ty_path(),
        entity_path_menu.f32_ty_path(),
        entity_path_menu.f64_ty_path(),
        entity_path_menu.trai_ty_path(),
    ];

    // Iterate over the type paths and assert that they are Ok
    for ty_path in ty_paths {
        let ty_decl = ty_path.decl(&db).unwrap();
        let ty_declarative_signature_template =
            db.ty_declarative_signature_template_from_decl(ty_decl);

        assert!(
            ty_declarative_signature_template.is_ok(),
            "Failed for type path: {:?}",
            ty_path
        );
    }
}
