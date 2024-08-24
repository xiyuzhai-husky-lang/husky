use crate::*;
use husky_entity_kind::MajorFormKind;
use husky_entity_path::path::{
    major_item::{form::MajorFormPath, MajorItemPath},
    ItemPath, ItemPathId,
};
use husky_entity_tree::helpers::paths::module_item_paths;
use husky_vfs::path::module_path::ModulePath;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct CodeLens {
    item_path: ItemPath,
    data: CodeLensData,
}

impl CodeLens {
    pub fn item_path(&self) -> ItemPath {
        self.item_path
    }

    pub fn data(&self) -> &CodeLensData {
        &self.data
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CodeLensData {
    Dep,
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
fn item_code_lenses_data(db: &::salsa::Db, item_path_id: ItemPathId) -> Vec<CodeLensData> {
    let item_path = item_path_id.item_path(db);
    match item_path {
        ItemPath::Submodule(_, _) => vec![],
        ItemPath::MajorItem(major_item_path) => match major_item_path {
            MajorItemPath::Type(_) => vec![],
            MajorItemPath::Trait(_) => vec![],
            MajorItemPath::Form(major_form_path) => {
                major_form_code_lenses_data(major_form_path, db)
            }
        },
        ItemPath::AssocItem(_) => vec![],
        ItemPath::TypeVariant(_, _) => vec![],
        ItemPath::ImplBlock(_) => vec![],
        ItemPath::Attr(_, _) => vec![],
        ItemPath::Chunk(_, _) => vec![],
    }
}

fn major_form_code_lenses_data(
    major_form_path: MajorFormPath,
    db: &::salsa::Db,
) -> Vec<CodeLensData> {
    match major_form_path.kind(db) {
        MajorFormKind::Ritchie(_) => vec![],
        MajorFormKind::TypeAlias => vec![],
        MajorFormKind::TypeVar => vec![],
        MajorFormKind::Val => vec![CodeLensData::Dep],
        MajorFormKind::StaticMut => vec![],
        MajorFormKind::StaticVar => vec![],
        MajorFormKind::Compterm => vec![],
        MajorFormKind::Conceptual => vec![],
    }
}
