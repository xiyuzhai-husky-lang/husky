use crate::*;
use husky_entity_path::*;
use husky_hir_defn::HirDefn;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum LinkageItemPath {
    Fugitive(FugitivePath),
    AssociatedItem(AssociatedItemPath),
}

impl LinkageItemPath {
    pub fn try_from_item_path(item_path: ItemPath) -> Option<Self> {
        match item_path {
            ItemPath::Submodule(_, _) => None,
            ItemPath::MajorItem(path) => match path {
                MajorItemPath::Type(_) => None,
                MajorItemPath::Trait(_) => None,
                MajorItemPath::Fugitive(path) => Some(path.into()),
            },
            ItemPath::AssociatedItem(path) => Some(path.into()),
            ItemPath::TypeVariant(_, _) => None,
            ItemPath::ImplBlock(_) => None,
            ItemPath::Attr(_, _) => None,
        }
    }

    pub fn hir_defn(self, db: &::salsa::Db) -> HirDefn {
        todo!()
    }
}
