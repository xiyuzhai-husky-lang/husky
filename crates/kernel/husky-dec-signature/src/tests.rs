pub(crate) use husky_ast::test_utils::*;

use crate::*;
use husky_corgi_config::CorgiConfigJar;
use husky_corgi_config_ast::CorgiConfigAstJar;
use husky_entity_tree::EntityTreeJar;
use husky_manifest::ManifestJar;
use husky_manifest_ast::ManifestAstJar;
use husky_syn_expr::jar::SynExprJar;
use husky_token::TokenJar;
use husky_toml_ast::TomlAstJar;
use husky_vfs::ModulePath;

#[salsa::db(
    CowordJar,
    husky_vfs::jar::VfsJar,
    husky_entity_path::jar::EntityPathJar,
    husky_token_data::jar::TokenDataJar,
    TokenJar,
    husky_ast::jar::AstJar,
    EntityTreeJar,
    husky_toml_token::jar::TomlTokenJar,
    TomlAstJar,
    ManifestAstJar,
    CorgiConfigJar,
    CorgiConfigAstJar,
    ManifestJar,
    SynExprJar,
    SynDeclJar,
    TermPreludeJar,
    husky_dec_term::jar::DecTermJar,
    DecSignatureJar
)]
#[derive(Default)]
pub(crate) struct DB;

fn module_dec_templates(
    db: &::salsa::Db,
    module_path: ModulePath,
) -> Vec<(ItemPath, DecSignatureResult<DecTemplate>)> {
    syn_decl_sheet(db, module_path)
        .decls(db)
        .iter()
        .copied()
        .map(|(path, _)| (path, path.dec_template(db)))
        .collect()
}

#[test]
fn module_dec_templates_works() {
    DB::ast_expect_test_debug_with_db(
        |db, module_path| module_dec_templates(db, module_path),
        &AstTestConfig::new(
            "module_declarative_templates",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::KERNEL,
        ),
    )
}

#[test]
fn menu_ty_dec_templates_works() {
    let db = DB::default();
    let db = &*db;
    let toolchain = db.dev_toolchain().unwrap();
    let item_path_menu = item_path_menu(db, toolchain);
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
        let ty_dec_template = ty_path.dec_template(db);
        assert!(
            ty_dec_template.is_ok(),
            "Failed for type path: {:?}",
            ty_path
        );
    }
}
