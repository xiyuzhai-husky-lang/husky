use super::*;
use crate::helpers::paths::HasItemPaths;
use husky_entity_kind::*;
use husky_entity_path::path::{ItemPath, ItemPathId};
use husky_task_interface::TaskIngredientIndex;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct IngredientPath {
    item_path: ItemPath,
}

pub trait HasIngredientPaths: is::Is<CratePath> {
    fn ingredient_paths<'a>(self, db: &'a ::salsa::Db) -> &'a [IngredientPath];
    fn has_ingredients(self, db: &::salsa::Db) -> bool {
        !self.ingredient_paths(db).is_empty()
    }
}

impl HasIngredientPaths for CratePath {
    fn ingredient_paths<'a>(self, db: &'a salsa::Db) -> &'a [IngredientPath] {
        crate_ingredient_paths(db, self)
    }
}

#[salsa::tracked(return_ref)]
pub(crate) fn crate_ingredient_paths(
    db: &::salsa::Db,
    crate_path: CratePath,
) -> Vec<IngredientPath> {
    crate_path
        .item_paths(db)
        .iter()
        .filter_map(|&item_path| IngredientPath::from_item_path(item_path, db))
        .collect()
}

impl IngredientPath {
    fn from_item_path(item_path: ItemPath, db: &::salsa::Db) -> Option<Self> {
        Self::is_ingredient(item_path, db).then_some(Self { item_path })
    }

    fn is_ingredient(path: ItemPath, db: &::salsa::Db) -> bool {
        match path.item_kind(db) {
            EntityKind::Module => false,
            EntityKind::MajorItem {
                module_item_kind,
                connection: _,
            } => match module_item_kind {
                MajorItemKind::Type(_) => false,
                MajorItemKind::Form(form_kind) => match form_kind {
                    MajorFormKind::Ritchie(ritchie_item_kind) => ritchie_item_kind.needs_jar(),
                    MajorFormKind::TypeAlias => false,
                    MajorFormKind::TypeVar => false,
                    MajorFormKind::Val => true,
                    MajorFormKind::Conceptual => false,
                    MajorFormKind::Static => true,
                    MajorFormKind::Compterm => true,
                },
                MajorItemKind::Trait => false,
            },
            EntityKind::AssocItem { assoc_item_kind } => match assoc_item_kind {
                AssocItemKind::TypeItem(
                    TypeItemKind::AssocRitchie(ritchie_item_kind)
                    | TypeItemKind::MethodRitchie(ritchie_item_kind),
                )
                | AssocItemKind::TraitItem(
                    TraitItemKind::AssocRitchie(ritchie_item_kind)
                    | TraitItemKind::MethodRitchie(ritchie_item_kind),
                )
                | AssocItemKind::TraitForTypeItem(
                    TraitItemKind::AssocRitchie(ritchie_item_kind)
                    | TraitItemKind::MethodRitchie(ritchie_item_kind),
                ) => ritchie_item_kind.needs_jar(),
                AssocItemKind::TraitItem(
                    TraitItemKind::MemoizedField | TraitItemKind::AssocVal,
                )
                | AssocItemKind::TypeItem(TypeItemKind::MemoizedField | TypeItemKind::AssocVal)
                | AssocItemKind::TraitForTypeItem(
                    TraitItemKind::MemoizedField | TraitItemKind::AssocVal,
                ) => true,
                _ => false,
            },
            EntityKind::TypeVariant => false,
            EntityKind::ImplBlock => false,
            EntityKind::Attr => false,
            EntityKind::Script => todo!(),
        }
    }

    pub fn item_path(self) -> ItemPath {
        self.item_path
    }
}

pub trait HasIngredientIndex: Into<ItemPath> {
    fn ingredient_index(self, db: &::salsa::Db) -> Option<TaskIngredientIndex>;
}

impl<P> HasIngredientIndex for P
where
    P: Into<ItemPath>,
{
    fn ingredient_index(self, db: &::salsa::Db) -> Option<TaskIngredientIndex> {
        item_path_ingredient_index(db, *self.into())
    }
}

#[salsa::tracked]
fn item_path_ingredient_index(
    db: &::salsa::Db,
    item_path_id: ItemPathId,
) -> Option<TaskIngredientIndex> {
    item_path_id
        .crate_path(db)
        .ingredient_paths(db)
        .iter()
        .position(|ingredient_path| *ingredient_path.item_path == item_path_id)
        .map(|raw| TaskIngredientIndex::from_index(raw))
}
