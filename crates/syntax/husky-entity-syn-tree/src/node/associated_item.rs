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
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
#[enum_class::from_variants]
pub enum AssociatedItemSynNodePath {
    TypeItem(TypeItemSynNodePath),
    TraitItem(TraitItemSynNodePath),
    TraitForTypeItem(TraitForTypeItemSynNodePath),
    IllFormedItem(IllFormedItemSynNodePath),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
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

    pub(crate) fn syn_node(self, _db: &::salsa::Db) -> AssociatedItemSynNode {
        todo!()
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

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
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
