mod trai_item;
mod ty_as_trai_item;
mod ty_item;

pub use trai_item::*;
pub use ty_as_trai_item::*;
pub use ty_item::*;

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = EntityPathDb)]
pub enum AssociatedItemPath {
    TypeItem(TypeItemPath),
    TraitItem(TraitItemPath),
    TypeAsTraitItem(TypeAsTraitItemPath),
}

impl AssociatedItemPath {
    pub fn ident(self, _db: &dyn EntityPathDb) -> Identifier {
        match self {
            AssociatedItemPath::TypeItem(_) => todo!(),
            AssociatedItemPath::TraitItem(_) => todo!(),
            AssociatedItemPath::TypeAsTraitItem(_) => todo!(),
        }
    }

    pub(crate) fn entity_kind(self, db: &dyn EntityPathDb) -> EntityKind {
        match self {
            AssociatedItemPath::TypeItem(path) => EntityKind::AssociatedItem {
                associated_item_kind: AssociatedItemKind::TypeItem(path.ty_item_kind(db)),
            },
            AssociatedItemPath::TraitItem(_) => todo!(),
            AssociatedItemPath::TypeAsTraitItem(_) => todo!(),
        }
    }
}

impl From<TypeAsTraitItemPath> for AssociatedItemPath {
    fn from(v: TypeAsTraitItemPath) -> Self {
        Self::TypeAsTraitItem(v)
    }
}

impl From<TraitItemPath> for AssociatedItemPath {
    fn from(v: TraitItemPath) -> Self {
        Self::TraitItem(v)
    }
}

impl From<TypeItemPath> for AssociatedItemPath {
    fn from(v: TypeItemPath) -> Self {
        Self::TypeItem(v)
    }
}

impl From<TraitItemPath> for EntityPath {
    fn from(v: TraitItemPath) -> Self {
        EntityPath::AssociatedItem(v.into())
    }
}

impl From<TypeItemPath> for EntityPath {
    fn from(v: TypeItemPath) -> Self {
        EntityPath::AssociatedItem(v.into())
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
            AssociatedItemPath::TypeAsTraitItem(path) => {
                path.display_with_db_fmt(f, db, level.next())
            }
        }
    }
}
