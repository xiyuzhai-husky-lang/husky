pub(crate) use husky_ast::test_utils::*;

use crate::*;
use husky_ast::AstJar;
use husky_corgi_config::CorgiConfigJar;
use husky_corgi_config_ast::CorgiConfigAstJar;
use husky_coword::CowordJar;
use husky_entity_path::EntityPathJar;
use husky_entity_syn_tree::EntitySynTreeJar;
use husky_manifest::ManifestJar;
use husky_manifest_ast::ManifestAstJar;
use husky_syn_expr::SynExprJar;
use husky_term_prelude::TermPreludeJar;
use husky_token::TokenJar;
use husky_toml_ast::TomlAstJar;
use husky_toml_token::TomlTokenJar;

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
    SynDeclJar,
    TermPreludeJar,
    DeclarativeTermJar,
    DeclarativeSignatureJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}

fn module_declarative_signature_templates(
    db: &DB,
    module_path: ModulePath,
) -> Vec<(ItemPath, DeclarativeSignatureResult<SignatureTemplate>)> {
    let Ok(syn_decl_sheet) = syn_decl_sheet(db, module_path) else {
        return vec![];
    };
    syn_decl_sheet
        .decls(db)
        .iter()
        .copied()
        .map(|(path, _)| (path, path.declarative_signature_template(db)))
        .collect()
}

#[test]
fn module_declarative_signature_templates_works() {
    DB::default().ast_expect_test_debug_with_db(
        "module_declarative_signature",
        module_declarative_signature_templates,
    )
}

#[test]
fn menu_ty_declarative_signature_templates_works() {
    let db = DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let item_path_menu = db.item_path_menu(toolchain);
    let ty_paths = vec![
        item_path_menu.i16_ty_path(),
        item_path_menu.i32_ty_path(),
        item_path_menu.i64_ty_path(),
        item_path_menu.u8_ty_path(),
        item_path_menu.u16_ty_path(),
        item_path_menu.u32_ty_path(),
        item_path_menu.u64_ty_path(),
        item_path_menu.f32_ty_path(),
        item_path_menu.f64_ty_path(),
        item_path_menu.trai_ty_path(),
    ];

    // Iterate over the type paths and assert that they are Ok
    for ty_path in ty_paths {
        let ty_declarative_signature_template = ty_path.declarative_signature_template(&db);
        assert!(
            ty_declarative_signature_template.is_ok(),
            "Failed for type path: {:?}",
            ty_path
        );
    }
}
