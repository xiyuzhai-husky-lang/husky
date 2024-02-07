pub mod ill_formed_item;
pub mod trai_for_ty_item;
pub mod trai_item;
pub mod ty_item;

pub use self::ill_formed_item::*;
pub use self::trai_for_ty_item::*;
pub use self::trai_item::*;
pub use self::ty_item::*;

use crate::*;
use husky_coword::IdentPairMap;
use husky_entity_kind::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum AssocItemSynNodePath {
    TypeItem(TypeItemSynNodePath),
    TraitItem(TraitItemSynNodePath),
    TraitForTypeItem(TraitForTypeItemSynNodePath),
    IllFormedItem(IllFormedItemSynNodePath),
}

#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[enum_class::from_variants]
pub enum AssocItemSynNodePathData {
    TypeItem(TypeItemSynNodePathData),
    TraitItem(TraitItemSynNodePathData),
    TraitForTypeItem(TraitForTypeItemSynNodePathData),
}

impl std::ops::Deref for AssocItemSynNodePath {
    type Target = ItemSynNodePathId;

    fn deref(&self) -> &Self::Target {
        let slf: &(u32, ItemSynNodePathId) = unsafe { std::mem::transmute(self) };
        &slf.1
    }
}

impl AssocItemSynNodePath {
    pub fn path(self, db: &::salsa::Db) -> Option<AssocItemPath> {
        Some(match (*self).path(db)? {
            ItemPath::AssocItem(path) => path,
            _ => unreachable!(),
        })
    }
}

impl AssocItemSynNodePathData {
    #[inline(always)]
    pub fn syn_node_path(self, id: ItemSynNodePathId) -> AssocItemSynNodePath {
        match self {
            AssocItemSynNodePathData::TypeItem(slf) => slf.syn_node_path(id).into(),
            AssocItemSynNodePathData::TraitItem(slf) => slf.syn_node_path(id).into(),
            AssocItemSynNodePathData::TraitForTypeItem(slf) => slf.syn_node_path(id).into(),
        }
    }

    pub fn path(self) -> Option<AssocItemPath> {
        match self {
            AssocItemSynNodePathData::TypeItem(slf) => slf.path().map(Into::into),
            AssocItemSynNodePathData::TraitItem(slf) => slf.path().map(Into::into),
            AssocItemSynNodePathData::TraitForTypeItem(slf) => slf.path().map(Into::into),
        }
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        match self {
            AssocItemSynNodePathData::TypeItem(slf) => slf.module_path(db),
            AssocItemSynNodePathData::TraitItem(slf) => slf.module_path(db),
            AssocItemSynNodePathData::TraitForTypeItem(slf) => slf.module_path(db),
        }
    }

    pub fn ast_idx(self, id: ItemSynNodePathId, db: &::salsa::Db) -> AstIdx {
        match self {
            AssocItemSynNodePathData::TypeItem(slf) => slf.ast_idx(id, db),
            AssocItemSynNodePathData::TraitItem(slf) => slf.ast_idx(id, db),
            AssocItemSynNodePathData::TraitForTypeItem(slf) => slf.ast_idx(id, db),
        }
    }
}

impl HasSynNodePath for AssocItemPath {
    type SynNodePath = AssocItemSynNodePath;

    fn syn_node_path(self, db: &::salsa::Db) -> Self::SynNodePath {
        match self {
            AssocItemPath::TypeItem(slf) => slf.syn_node_path(db).into(),
            AssocItemPath::TraitItem(slf) => slf.syn_node_path(db).into(),
            AssocItemPath::TraitForTypeItem(slf) => slf.syn_node_path(db).into(),
        }
    }
}
