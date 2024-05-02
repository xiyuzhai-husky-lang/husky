pub mod trai_for_ty_item;
pub mod trai_item;
pub mod ty_item;

use self::trai_for_ty_item::*;
use self::trai_item::*;
use self::ty_item::*;
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum AssocItemPath {
    TypeItem(TypeItemPath),
    TraitItem(TraitItemPath),
    TraitForTypeItem(TraitForTypeItemPath),
}

impl std::ops::Deref for AssocItemPath {
    type Target = ItemPathId;

    fn deref(&self) -> &Self::Target {
        unsafe { &std::mem::transmute::<_, &(u32, ItemPathId)>(self).1 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum AssocItemPathData {
    TypeItem(TypeItemPathData),
    TraitItem(TraitItemPathData),
    TraitForTypeItem(TraitForTypeItemPathData),
}

impl From<TraitItemPath> for ItemPath {
    fn from(v: TraitItemPath) -> Self {
        ItemPath::AssocItem(v.into())
    }
}

impl From<TypeItemPath> for ItemPath {
    fn from(v: TypeItemPath) -> Self {
        ItemPath::AssocItem(v.into())
    }
}

impl AssocItemPathData {
    #[inline(always)]
    pub(super) fn item_path(self, id: ItemPathId) -> AssocItemPath {
        match self {
            AssocItemPathData::TypeItem(slf) => slf.item_path(id).into(),
            AssocItemPathData::TraitItem(slf) => slf.item_path(id).into(),
            AssocItemPathData::TraitForTypeItem(slf) => slf.item_path(id).into(),
        }
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        match self {
            AssocItemPathData::TypeItem(data) => data.module_path(db),
            AssocItemPathData::TraitItem(data) => data.module_path(db),
            AssocItemPathData::TraitForTypeItem(data) => data.module_path(db),
        }
    }

    pub fn ident(self, _db: &::salsa::Db) -> Ident {
        match self {
            AssocItemPathData::TypeItem(slf) => slf.ident,
            AssocItemPathData::TraitItem(slf) => slf.ident,
            AssocItemPathData::TraitForTypeItem(slf) => slf.ident,
        }
    }

    pub(crate) fn entity_kind(self, db: &::salsa::Db) -> EntityKind {
        match self {
            AssocItemPathData::TypeItem(slf) => slf.entity_kind(db),
            AssocItemPathData::TraitItem(slf) => slf.entity_kind(db),
            AssocItemPathData::TraitForTypeItem(slf) => slf.entity_kind(db),
        }
    }
}

impl salsa::DisplayWithDb for AssocItemPath {
    fn display_fmt_with_db(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        match self {
            AssocItemPath::TypeItem(path) => path.display_fmt_with_db(f, db),
            AssocItemPath::TraitItem(path) => path.display_fmt_with_db(f, db),
            AssocItemPath::TraitForTypeItem(path) => path.display_fmt_with_db(f, db),
        }
    }
}
