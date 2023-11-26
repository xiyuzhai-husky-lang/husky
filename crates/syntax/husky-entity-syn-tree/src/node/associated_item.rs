mod ill_formed_item;
mod trai_for_ty_item;
mod trai_item;
mod ty_item;

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

impl AssociatedItemSynNodePath {
    pub fn path(self, db: &::salsa::Db) -> Option<AssociatedItemPath> {
        match self {
            AssociatedItemSynNodePath::TypeItem(syn_node_path) => {
                syn_node_path.path(db).map(Into::into)
            }
            AssociatedItemSynNodePath::TraitItem(syn_node_path) => {
                syn_node_path.path(db).map(Into::into)
            }
            AssociatedItemSynNodePath::TraitForTypeItem(syn_node_path) => {
                syn_node_path.path(db).map(Into::into)
            }
            AssociatedItemSynNodePath::IllFormedItem(_syn_node_path) => None,
        }
    }

    pub(crate) fn syn_node(self, _db: &::salsa::Db) -> AssociatedItemSynNode {
        todo!()
    }
}

// impl HasModulePath<Db> for AssociatedItemSynNodePath
// where
//      + EntitySynTreeDb,
// {
//     fn module_path(self, db: &::salsa::Db,) -> ModulePath {
//         match self {
//             AssociatedItemSynNodePath::TypeItem(syn_node_path) => syn_node_path.module_path(db),
//             AssociatedItemSynNodePath::TraitItem(syn_node_path) => syn_node_path.module_path(db),
//             AssociatedItemSynNodePath::TraitForTypeItem(syn_node_path) => {
//                 syn_node_path.module_path(db)
//             }
//             AssociatedItemSynNodePath::IllFormedItem(syn_node_path) => {
//                 syn_node_path.module_path(db)
//             }
//         }
//     }
// }

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub(crate) enum AssociatedItemSynNode {
    TypeItem(TypeItemSynNode),
    TraitItem(TraitItemSynNode),
    TraitForTypeItem(TraitForTypeItemSynNode),
}

impl AssociatedItemSynNode {
    pub fn syn_node_path(self, db: &::salsa::Db) -> AssociatedItemSynNodePath {
        match self {
            AssociatedItemSynNode::TypeItem(node) => node.syn_node_path(db).into(),
            AssociatedItemSynNode::TraitItem(_) => todo!(),
            AssociatedItemSynNode::TraitForTypeItem(_) => todo!(),
        }
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        match self {
            AssociatedItemSynNode::TypeItem(node) => node.module_path(db),
            AssociatedItemSynNode::TraitItem(_) => todo!(),
            AssociatedItemSynNode::TraitForTypeItem(_) => todo!(),
        }
    }
}
