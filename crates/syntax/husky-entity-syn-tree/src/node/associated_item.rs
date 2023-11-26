mod trai_for_ty_item;
mod trai_item;
mod ty_item;

pub use self::trai_for_ty_item::*;
pub use self::trai_item::*;
pub use self::ty_item::*;

use crate::*;
use husky_coword::IdentPairMap;
use husky_entity_kind::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
#[enum_class::from_variants]
pub enum AssociatedItemSynNodeDataPath {
    TypeItem(TypeItemSynNodePath),
    TraitItem(TraitItemSynNodePath),
    TraitForTypeItem(TraitForTypeItemSynNodePath),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
#[enum_class::from_variants]
pub enum AssociatedItemSynNodeDataPathData {
    TypeItem(TypeItemSynNodePathData),
    TraitItem(TraitItemSynNodePathData),
    TraitForTypeItem(TraitForTypeItemSynNodePathData),
}

impl AssociatedItemSynNodeDataPath {
    pub fn path(self, db: &::salsa::Db) -> Option<AssociatedItemPath> {
        match self {
            AssociatedItemSynNodeDataPath::TypeItem(syn_node_path) => {
                syn_node_path.path(db).map(Into::into)
            }
            AssociatedItemSynNodeDataPath::TraitItem(syn_node_path) => {
                syn_node_path.path(db).map(Into::into)
            }
            AssociatedItemSynNodeDataPath::TraitForTypeItem(syn_node_path) => {
                syn_node_path.path(db).map(Into::into)
            }
            AssociatedItemSynNodeDataPath::IllFormedItem(_syn_node_path) => None,
        }
    }

    pub(crate) fn syn_node(self, _db: &::salsa::Db) -> AssociatedItemSynNodeData {
        todo!()
    }
}

impl HasSynNodePath for AssociatedItemPath {
    type SynNodePath = AssociatedItemSynNodeDataPath;

    fn syn_node_path(self, db: &::salsa::Db) -> Self::SynNodePath {
        match self {
            AssociatedItemPath::TypeItem(slf) => slf.syn_node_path(db).into(),
            AssociatedItemPath::TraitItem(slf) => slf.syn_node_path(db).into(),
            AssociatedItemPath::TraitForTypeItem(slf) => slf.syn_node_path(db).into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub(crate) enum AssociatedItemSynNodeData {
    TypeItem(TypeItemSynNodeData),
    TraitItem(TraitItemSynNodeData),
    TraitForTypeItem(TraitForTypeItemSynNodeData),
}

impl AssociatedItemSynNodeData {
    pub fn syn_node_path(self, db: &::salsa::Db) -> AssociatedItemSynNodeDataPath {
        match self {
            AssociatedItemSynNodeData::TypeItem(node) => node.syn_node_path(db).into(),
            AssociatedItemSynNodeData::TraitItem(_) => todo!(),
            AssociatedItemSynNodeData::TraitForTypeItem(_) => todo!(),
        }
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        match self {
            AssociatedItemSynNodeData::TypeItem(node) => node.module_path(db),
            AssociatedItemSynNodeData::TraitItem(_) => todo!(),
            AssociatedItemSynNodeData::TraitForTypeItem(_) => todo!(),
        }
    }
}
