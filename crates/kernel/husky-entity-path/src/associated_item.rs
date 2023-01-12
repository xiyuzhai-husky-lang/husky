mod trai_item;
mod ty_as_trai_item;
mod ty_item;

pub use trai_item::*;
pub use ty_as_trai_item::*;
pub use ty_item::*;

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssociatedItemPath {
    TypeItem(TypeItemPath),
    TraitItem(TraitItemPath),
    TypeAsTraitItem(TypeAsTraitItemPath),
}

impl AssociatedItemPath {
    pub fn ident(self, db: &dyn EntityPathDb) -> Identifier {
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

impl<Db> salsa::DebugWithDb<Db> for AssociatedItemPath
where
    Db: salsa::DbWithJar<EntityPathJar> + ?Sized,
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<EntityPathJar>>::as_jar_db(db);
        if include_all_fields {
            match self {
                AssociatedItemPath::TypeItem(path) => f
                    .debug_tuple("Type")
                    .field(&path.debug_with(db, include_all_fields))
                    .finish(),
                AssociatedItemPath::TraitItem(path) => f
                    .debug_tuple("Trait")
                    .field(&path.debug_with(db, include_all_fields))
                    .finish(),
                AssociatedItemPath::TypeAsTraitItem(path) => f
                    .debug_tuple("TypeAsTrait")
                    .field(&path.debug_with(db, include_all_fields))
                    .finish(),
            }
        } else {
            match self {
                AssociatedItemPath::TypeItem(path) => path.fmt(f, db, false),
                AssociatedItemPath::TraitItem(path) => path.fmt(f, db, false),
                AssociatedItemPath::TypeAsTraitItem(path) => path.fmt(f, db, false),
            }
        }
    }
}
