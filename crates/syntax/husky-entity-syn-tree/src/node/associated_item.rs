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
pub enum AssociatedItemSynNodePath {
    TypeItem(TypeItemSynNodePath),
    TraitItem(TraitItemSynNodePath),
    TraitForTypeItem(TraitForTypeItemSynNodePath),
    IllFormedItem(IllFormedItemSynNodePath),
}

#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[enum_class::from_variants]
pub enum AssociatedItemSynNodePathData {
    TypeItem(TypeItemSynNodePathData),
    TraitItem(TraitItemSynNodePathData),
    TraitForTypeItem(TraitForTypeItemSynNodePathData),
}

impl std::ops::Deref for AssociatedItemSynNodePath {
    type Target = ItemSynNodePathId;

    fn deref(&self) -> &Self::Target {
        let slf: &(u32, ItemSynNodePathId) = unsafe { std::mem::transmute(self) };
        &slf.1
    }
}

impl AssociatedItemSynNodePath {
    pub fn path(self, db: &::salsa::Db) -> Option<AssociatedItemPath> {
        Some(match (*self).path(db)? {
            ItemPath::AssociatedItem(path) => path,
            _ => unreachable!(),
        })
    }
}

impl AssociatedItemSynNodePathData {
    #[inline(always)]
    pub fn syn_node_path(self, id: ItemSynNodePathId) -> AssociatedItemSynNodePath {
        match self {
            AssociatedItemSynNodePathData::TypeItem(slf) => slf.syn_node_path(id).into(),
            AssociatedItemSynNodePathData::TraitItem(slf) => slf.syn_node_path(id).into(),
            AssociatedItemSynNodePathData::TraitForTypeItem(slf) => slf.syn_node_path(id).into(),
        }
    }

    pub fn path(self) -> Option<AssociatedItemPath> {
        match self {
            AssociatedItemSynNodePathData::TypeItem(slf) => slf.path().map(Into::into),
            AssociatedItemSynNodePathData::TraitItem(slf) => slf.path().map(Into::into),
            AssociatedItemSynNodePathData::TraitForTypeItem(slf) => slf.path().map(Into::into),
        }
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        match self {
            AssociatedItemSynNodePathData::TypeItem(slf) => slf.module_path(db),
            AssociatedItemSynNodePathData::TraitItem(slf) => slf.module_path(db),
            AssociatedItemSynNodePathData::TraitForTypeItem(slf) => slf.module_path(db),
        }
    }

    pub fn ast_idx(self, id: ItemSynNodePathId, db: &::salsa::Db) -> AstIdx {
        match self {
            AssociatedItemSynNodePathData::TypeItem(slf) => slf.ast_idx(id, db),
            AssociatedItemSynNodePathData::TraitItem(slf) => slf.ast_idx(id, db),
            AssociatedItemSynNodePathData::TraitForTypeItem(slf) => slf.ast_idx(id, db),
        }
    }
}

impl HasSynNodePath for AssociatedItemPath {
    type SynNodePath = AssociatedItemSynNodePath;

    fn syn_node_path(self, db: &::salsa::Db) -> Self::SynNodePath {
        match self {
            AssociatedItemPath::TypeItem(slf) => slf.syn_node_path(db).into(),
            AssociatedItemPath::TraitItem(slf) => slf.syn_node_path(db).into(),
            AssociatedItemPath::TraitForTypeItem(slf) => slf.syn_node_path(db).into(),
        }
    }
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum AssociatedItemSynNode {
    TypeItem(TypeItemSynNode),
    TraitItem(TraitItemSynNode),
    TraitForTypeItem(TraitForTypeItemSynNode),
}

impl AssociatedItemSynNode {
    pub fn syn_node_path(&self) -> AssociatedItemSynNodePath {
        match self {
            AssociatedItemSynNode::TypeItem(node) => node.syn_node_path.into(),
            AssociatedItemSynNode::TraitItem(_) => todo!(),
            AssociatedItemSynNode::TraitForTypeItem(_) => todo!(),
        }
    }
}
