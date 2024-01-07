pub mod connection;
pub mod fugitive;
pub mod trai;
pub mod ty;
mod utils;

pub use self::connection::*;
pub use self::fugitive::*;
pub use self::trai::*;
pub use self::ty::*;

use crate::*;

use utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum MajorItemPath {
    Type(TypePath),
    Trait(TraitPath),
    Fugitive(FugitivePath),
}

impl std::ops::Deref for MajorItemPath {
    type Target = ItemPathId;

    fn deref(&self) -> &Self::Target {
        unsafe { &std::mem::transmute::<_, &(u32, ItemPathId)>(self).1 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum MajorItemPathData {
    Type(TypePathData),
    Trait(TraitPathData),
    Fugitive(FugitivePathData),
}

impl MajorItemPath {
    pub fn data(self, db: &::salsa::Db) -> MajorItemPathData {
        match (*self).data(db) {
            ItemPathData::MajorItem(data) => data,
            _ => unreachable!(),
        }
    }

    pub fn ty_path(self) -> Option<TypePath> {
        match self {
            MajorItemPath::Type(data) => Some(data),
            MajorItemPath::Trait(_) | MajorItemPath::Fugitive(_) => None,
        }
    }

    pub fn ident(self, db: &::salsa::Db) -> Ident {
        self.data(db).ident()
    }
}

impl MajorItemPathData {
    #[inline(always)]
    pub(super) fn item_path(self, id: ItemPathId) -> MajorItemPath {
        match self {
            MajorItemPathData::Type(slf) => slf.item_path(id).into(),
            MajorItemPathData::Trait(slf) => slf.item_path(id).into(),
            MajorItemPathData::Fugitive(slf) => slf.item_path(id).into(),
        }
    }

    pub fn module_path(self, _db: &::salsa::Db) -> ModulePath {
        match self {
            MajorItemPathData::Type(data) => data.module_path(),
            MajorItemPathData::Trait(data) => data.module_path(),
            MajorItemPathData::Fugitive(data) => data.module_path(),
        }
    }
    pub fn ident(self) -> Ident {
        match self {
            MajorItemPathData::Type(data) => data.ident(),
            MajorItemPathData::Trait(data) => data.ident(),
            MajorItemPathData::Fugitive(data) => data.ident(),
        }
    }

    pub fn crate_path(self, db: &::salsa::Db) -> CratePath {
        self.module_path(db).crate_path(db)
    }

    pub(crate) fn entity_kind(self, _db: &::salsa::Db) -> EntityKind {
        match self {
            MajorItemPathData::Type(data) => data.item_kind(),
            MajorItemPathData::Trait(data) => EntityKind::MajorItem {
                module_item_kind: MajorItemKind::Trait,
                connection: data.connection().kind(),
            },
            MajorItemPathData::Fugitive(data) => EntityKind::MajorItem {
                module_item_kind: MajorItemKind::Fugitive(data.fugitive_kind()),
                connection: data.connection().kind(),
            },
        }
    }
}

impl salsa::DisplayWithDb for MajorItemPath {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        match self {
            MajorItemPath::Fugitive(path) => path.display_with_db_fmt(f, db),
            MajorItemPath::Type(path) => path.display_with_db_fmt(f, db),
            MajorItemPath::Trait(path) => path.display_with_db_fmt(f, db),
        }
    }
}
