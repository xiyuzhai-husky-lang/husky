mod trai_for_ty_item;
mod trai_item;
mod ty_item;

pub use trai_for_ty_item::*;
pub use trai_item::*;
pub use ty_item::*;

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = EntityPathDb)]
#[enum_class::from_variants]
pub enum AssociatedItemPath {
    TypeItem(TypeItemPath),
    TraitItem(TraitItemPath),
    TraitForTypeItem(TraitForTypeItemPath),
}

impl AssociatedItemPath {
    pub fn ident(self, db: &dyn EntityPathDb) -> Ident {
        match self {
            AssociatedItemPath::TypeItem(path) => path.ident(db),
            AssociatedItemPath::TraitItem(path) => path.ident(db),
            AssociatedItemPath::TraitForTypeItem(path) => path.ident(db),
        }
    }

    pub fn module_path(self, db: &dyn EntityPathDb) -> ModulePath {
        match self {
            AssociatedItemPath::TypeItem(path) => path.module_path(db),
            AssociatedItemPath::TraitItem(path) => path.module_path(db),
            AssociatedItemPath::TraitForTypeItem(path) => path.module_path(db),
        }
    }

    pub(crate) fn item_kind(self, db: &dyn EntityPathDb) -> EntityKind {
        EntityKind::AssociatedItem {
            associated_item_kind: match self {
                AssociatedItemPath::TypeItem(path) => {
                    AssociatedItemKind::TypeItem(path.item_kind(db))
                }

                AssociatedItemPath::TraitItem(path) => {
                    AssociatedItemKind::TraitItem(path.item_kind(db))
                }
                AssociatedItemPath::TraitForTypeItem(path) => {
                    AssociatedItemKind::TraitForTypeItem(path.item_kind(db))
                }
            },
        }
    }
}

impl From<TraitItemPath> for ItemPath {
    fn from(v: TraitItemPath) -> Self {
        ItemPath::AssociatedItem(v.into())
    }
}

impl From<TypeItemPath> for ItemPath {
    fn from(v: TypeItemPath) -> Self {
        ItemPath::AssociatedItem(v.into())
    }
}

impl<Db> salsa::DisplayWithDb<Db> for AssociatedItemPath
where
    Db: EntityPathDb + ?Sized,
{
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        match self {
            AssociatedItemPath::TypeItem(path) => path.display_with_db_fmt(f, db, level.next()),
            AssociatedItemPath::TraitItem(path) => path.display_with_db_fmt(f, db, level.next()),
            AssociatedItemPath::TraitForTypeItem(path) => {
                path.display_with_db_fmt(f, db, level.next())
            }
        }
    }
}
