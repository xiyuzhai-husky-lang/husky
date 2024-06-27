use crate::*;
use husky_entity_path::path::{ItemPath, ItemPathId};
use husky_entity_tree::helpers::paths::module_item_paths;
use husky_vfs::path::module_path::ModulePath;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct CodeLens {
    item_path: ItemPath,
    data: CodeLensData,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CodeLensData {
    Deps,
    Affect,
    Runnable,
    HasImpls,
    HasReferences,
}

#[salsa::tracked(return_ref)]
pub fn module_code_lenses(db: &::salsa::Db, module_path: ModulePath) -> Vec<CodeLens> {
    let mut code_lenses = vec![];
    for &item_path in module_item_paths(db, module_path) {
        code_lenses.extend(
            item_code_lenses_data(db, *item_path)
                .iter()
                .map(|data| CodeLens {
                    item_path,
                    data: data.clone(),
                }),
        );
    }
    code_lenses
}

#[test]
fn module_code_lenses_works() {
    DB::ast_rich_test_debug_with_db(
        module_code_lenses,
        &AstTestConfig::new(
            "module_code_lens",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::IDE,
        ),
    );
}

#[salsa::tracked(return_ref)]
pub fn item_code_lenses_data(db: &::salsa::Db, item_path_id: ItemPathId) -> Vec<CodeLensData> {
    let item_path = item_path_id.item_path(db);
    match item_path {
        ItemPath::Submodule(_, _) => vec![],
        ItemPath::MajorItem(_) => vec![],
        ItemPath::AssocItem(_) => vec![],
        ItemPath::TypeVariant(_, _) => vec![],
        ItemPath::ImplBlock(_) => vec![],
        ItemPath::Attr(_, _) => vec![],
        ItemPath::Chunk(_, _) => vec![],
    }
}
