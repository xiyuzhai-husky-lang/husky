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
