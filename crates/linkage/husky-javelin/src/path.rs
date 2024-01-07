use crate::*;
use either::*;
use husky_entity_kind::TypeKind;
use husky_entity_path::*;
use husky_entity_syn_tree::helpers::paths::module_item_paths;
use husky_hir_defn::HirDefn;

#[salsa::debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum JavelinPath {
    Fugitive(FugitivePath),
    TypeItem(TypeItemPath),
    TraitItem(TraitItemPath),
    TraitForTypeItem(TraitForTypeItemPath),
    TypeConstructor(TypePath),
    TypeVariantConstructor(TypeVariantPath),
}

impl std::ops::Deref for JavelinPath {
    type Target = ItemPathId;

    fn deref(&self) -> &Self::Target {
        unsafe { &std::mem::transmute::<_, &(u32, ItemPathId)>(self).1 }
    }
}

#[test]
fn javelin_item_path_deref_works() {
    DB::default().ast_plain_test(
        |db, module_path| {
            for &item_path in module_item_paths(db, module_path) {
                if let Some(javelin_item_path) = JavelinPath::try_from_item_path(item_path, db) {
                    assert_eq!(*javelin_item_path, *item_path)
                }
            }
        },
        &AstTestConfig::new("javelin_item_path_deref"),
    )
}

impl JavelinPath {
    pub fn try_from_item_path(item_path: ItemPath, db: &::salsa::Db) -> Option<Self> {
        match item_path {
            ItemPath::Submodule(_, _) => None,
            ItemPath::MajorItem(path) => match path {
                MajorItemPath::Type(path) => match path.ty_kind(db) {
                    TypeKind::Struct | TypeKind::Enum => Some(path.into()),
                    TypeKind::Inductive
                    | TypeKind::Record
                    | TypeKind::Structure
                    | TypeKind::Extern => None,
                },
                MajorItemPath::Trait(_) => None,
                MajorItemPath::Fugitive(path) => Some(path.into()),
            },
            ItemPath::AssociatedItem(path) => match path {
                AssociatedItemPath::TraitForTypeItem(path) => {
                    if path.impl_block(db).trai_path(db).refine(db)
                        == Left(PreludeTraitPath::Visualize)
                    {
                        return None;
                    }
                    Some(JavelinPath::TraitForTypeItem(path))
                }
                AssociatedItemPath::TypeItem(path) => Some(JavelinPath::TypeItem(path)),
                AssociatedItemPath::TraitItem(path) => Some(JavelinPath::TraitItem(path)),
            },
            ItemPath::TypeVariant(_, path) => Some(path.into()),
            ItemPath::ImplBlock(_) => None,
            ItemPath::Attr(_, _) => None,
        }
    }

    pub fn hir_defn(self, _db: &::salsa::Db) -> HirDefn {
        todo!()
    }
}
