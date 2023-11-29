use crate::*;
use husky_entity_path::*;
use husky_entity_syn_tree::helpers::paths::module_item_paths;
use husky_hir_defn::HirDefn;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum LinkageItemPath {
    Fugitive(FugitivePath),
    TypeItem(TypeItemPath),
    TraitItem(TraitItemPath),
    TraitForTypeItem(TraitForTypeItemPath),
    TypeConstructor(TypePath),
    TypeVariantConstructor(TypeVariantPath),
}

impl std::ops::Deref for LinkageItemPath {
    type Target = ItemPathId;

    fn deref(&self) -> &Self::Target {
        unsafe { &std::mem::transmute::<_, &(u32, ItemPathId)>(self).1 }
    }
}

#[test]
fn linkage_item_path_deref_works() {
    DB::default().ast_plain_test(
        |db, module_path| {
            for &item_path in module_item_paths(db, module_path) {
                if let Some(linkage_item_path) = LinkageItemPath::try_from_item_path(item_path) {
                    assert_eq!(*linkage_item_path, *item_path)
                }
            }
        },
        &AstTestConfig::new("linkage_item_path_deref"),
    )
}

impl From<AssociatedItemPath> for LinkageItemPath {
    fn from(path: AssociatedItemPath) -> Self {
        match path {
            AssociatedItemPath::TypeItem(path) => path.into(),
            AssociatedItemPath::TraitItem(path) => path.into(),
            AssociatedItemPath::TraitForTypeItem(path) => path.into(),
        }
    }
}

impl LinkageItemPath {
    pub fn try_from_item_path(item_path: ItemPath) -> Option<Self> {
        match item_path {
            ItemPath::Submodule(_, _) => None,
            ItemPath::MajorItem(path) => match path {
                MajorItemPath::Type(path) => Some(path.into()),
                MajorItemPath::Trait(_) => None,
                MajorItemPath::Fugitive(path) => Some(path.into()),
            },
            ItemPath::AssociatedItem(path) => Some(path.into()),
            ItemPath::TypeVariant(_, path) => Some(path.into()),
            ItemPath::ImplBlock(_) => None,
            ItemPath::Attr(_, _) => None,
        }
    }

    pub fn hir_defn(self, db: &::salsa::Db) -> HirDefn {
        todo!()
    }
}
