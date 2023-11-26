mod trai_for_ty_item;
mod trai_item;
mod ty_item;

pub use trai_for_ty_item::*;
pub use trai_item::*;
pub use ty_item::*;

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = EntityPathDb, jar = EntityPathJar)]
#[enum_class::from_variants]
pub enum AssociatedItemPath {
    TypeItem(TypeItemPath),
    TraitItem(TraitItemPath),
    TraitForTypeItem(TraitForTypeItemPath),
}

impl std::ops::Deref for AssociatedItemPath {
    type Target = ItemPathId;

    fn deref(&self) -> &Self::Target {
        unsafe { &std::mem::transmute::<_, &(u32, ItemPathId)>(self).1 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = EntityPathDb, jar = EntityPathJar)]
#[enum_class::from_variants]
pub enum AssociatedItemPathData {
    TypeItem(TypeItemPathData),
    TraitItem(TraitItemPathData),
    TraitForTypeItem(TraitForTypeItemPathData),
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

impl AssociatedItemPathData {
    pub fn module_path(self, db: &dyn EntityPathDb) -> ModulePath {
        match self {
            AssociatedItemPathData::TypeItem(data) => data.module_path(db),
            AssociatedItemPathData::TraitItem(data) => data.module_path(db),
            AssociatedItemPathData::TraitForTypeItem(data) => data.module_path(db),
        }
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
