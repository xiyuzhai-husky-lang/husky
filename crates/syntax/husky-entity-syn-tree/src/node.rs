mod associated_item;
mod attr;
mod impl_block;
mod major_item;
mod submodule;
mod ty_variant;

pub use self::associated_item::*;
pub use self::attr::*;
pub use self::impl_block::*;
pub use self::major_item::*;
pub use self::submodule::*;
pub use self::ty_variant::*;

use crate::*;
use enum_class::Room32;
use husky_token::IdentToken;
use vec_like::VecPairMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
#[enum_class::from_variants]
pub enum ItemSynNodePath {
    Submodule(Room32, SubmoduleSynNodePath),
    MajorItem(MajorItemSynNodePath),
    TypeVariant(Room32, TypeVariantSynNodePath),
    ImplBlock(ImplBlockSynNodePath),
    AssociatedItem(AssociatedItemSynNodeDataPath),
    Attr(Room32, AttrSynNodePath),
}

impl std::ops::Deref for ItemSynNodePath {
    type Target = ItemSynNodePathId;

    fn deref(&self) -> &Self::Target {
        let slf: &(u32, u32, ItemSynNodePathId) = unsafe { std::mem::transmute(self) };
        &slf.2
    }
}

#[salsa::interned(jar = EntitySynTreeJar)]
pub struct ItemSynNodePathId {
    data: ItemSynNodePathData,
}

pub enum ItemSynNodePathData {
    Submodule(SubmoduleSynNodePathData),
    MajorItem(MajorItemSynNodePathData),
    TypeVariant(TypeVariantSynNodePathData),
    ImplBlock(ImplBlockSynNodePathData),
    AssociatedItem(AssociatedItemSynNodeDataPathData),
    Attr(AttrSynNodePath),
}

impl ItemSynNodePathId {
    pub fn syn_node(self, db: &::salsa::Db) -> ItemSynNode {
        todo!()
    }

    pub fn module_path(self, db: &dyn EntitySynTreeDb) -> ModulePath {
        todo!()
    }
}

pub struct ItemSynNodeId {
    data: ItemSynNodeData,
}

pub enum ItemSynNodeData {
    Submodule(SubmoduleSynNodeData),
    MajorItem(MajorItemSynNodeData),
    TypeVariant(TypeVariantSynNodeData),
    ImplBlock(ImplBlockSynNodeData),
    AssociatedItem(AssociatedItemSynNodeData),
    Attr(AttrSynNodeData),
}

impl ItemSynNodePath {
    pub fn path(self, db: &::salsa::Db) -> Option<ItemPath> {
        match self {
            ItemSynNodePath::Submodule(syn_node_path) => syn_node_path.path(db).map(Into::into),
            ItemSynNodePath::MajorItem(syn_node_path) => syn_node_path.path(db).map(Into::into),
            ItemSynNodePath::TypeVariant(syn_node_path) => syn_node_path.path(db).map(Into::into),
            ItemSynNodePath::ImplBlock(syn_node_path) => syn_node_path.path(db).map(Into::into),
            ItemSynNodePath::AssociatedItem(syn_node_path) => {
                syn_node_path.path(db).map(Into::into)
            }
            ItemSynNodePath::Attr(syn_node_path) => syn_node_path.path(db).map(Into::into),
        }
    }

    pub fn toolchain(self, db: &::salsa::Db) -> Toolchain {
        // self.module_path(db).toolchain(db)
        todo!()
    }

    pub(crate) fn attr_syn_nodes(self, db: &::salsa::Db) -> &[(AttrSynNodePath, AttrSynNodeData)] {
        // ad hoc
        match self {
            ItemSynNodePath::Submodule(_) => &[],
            ItemSynNodePath::MajorItem(path) => path.attrs(db),
            ItemSynNodePath::TypeVariant(_) => &[],
            ItemSynNodePath::ImplBlock(_) => &[],
            ItemSynNodePath::AssociatedItem(_) => &[],
            ItemSynNodePath::Attr(_) => &[],
        }
    }
}

pub trait HasSynNodePath: Copy {
    type SynNodePath;

    fn syn_node_path(self, db: &::salsa::Db) -> Self::SynNodePath;
}

impl HasSynNodePath for ItemPath {
    type SynNodePath = ItemSynNodePath;

    fn syn_node_path(self, db: &::salsa::Db) -> Self::SynNodePath {
        match self {
            ItemPath::Submodule(_, path) => path.syn_node_path(db).into(),
            ItemPath::MajorItem(path) => path.syn_node_path(db).into(),
            ItemPath::AssociatedItem(path) => path.syn_node_path(db).into(),
            ItemPath::TypeVariant(_, path) => path.syn_node_path(db).into(),
            ItemPath::ImplBlock(path) => path.syn_node_path(db).into(),
            ItemPath::Attr(_, path) => path.syn_node_path(db).into(),
        }
    }
}

#[derive(Default)]
pub(crate) struct ItemSynNodePathRegistry {
    next_disambiguators: VecPairMap<ItemPath, u8>,
    errors: Vec<EntitySynTreeError>,
}

impl ItemSynNodePathRegistry {
    pub(crate) fn finish_with_errors(self) -> Vec<EntitySynTreeError> {
        self.errors
    }

    fn issue_maybe_ambiguous_path<P: Copy + Into<ItemPath>>(
        &mut self,
        path: P,
    ) -> MaybeAmbiguousPath<P> {
        let next_disambiguator = self
            .next_disambiguators
            .get_value_mut_or_insert_default(path.into());
        let disambiguator = *next_disambiguator;
        *next_disambiguator += 1;
        MaybeAmbiguousPath {
            path,
            disambiguator,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub struct MaybeAmbiguousPath<P> {
    path: P,
    disambiguator: u8,
}

impl<P> MaybeAmbiguousPath<P> {
    fn from_path(path: P) -> Self {
        Self {
            path,
            disambiguator: 0,
        }
    }

    fn unambiguous_path(self) -> Option<P> {
        (self.disambiguator == 0).then_some(self.path)
    }
}

/// this is pub(crate) because it contains AstIdx which affects incremental computation
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
#[enum_class::from_variants]
pub(crate) enum ItemSynNode {
    Submodule(SubmoduleSynNodeData),
    MajorItem(MajorItemSynNodeData),
    AssociatedItem(AssociatedItemSynNodeData),
    TypeVariant(TypeVariantSynNodeData),
    ImplBlock(ImplBlockSynNodeData),
}

impl ItemSynNode {
    pub(crate) fn try_new(
        db: &::salsa::Db,
        registry: &mut ItemSynNodePathRegistry,
        visibility: Scope,
        ast_idx: AstIdx,
        ident_token: IdentToken,
        item_path: ItemPath,
        block: DefnBlock,
    ) -> Option<Self> {
        match item_path {
            ItemPath::Submodule(_, submodule_path) => Some(
                SubmoduleSynNodeData::new(
                    db,
                    registry,
                    submodule_path,
                    visibility,
                    ast_idx,
                    ident_token,
                )
                .into(),
            ),
            ItemPath::MajorItem(module_item_path) => Some(
                MajorItemSynNodeData::new(
                    db,
                    registry,
                    module_item_path,
                    visibility,
                    ast_idx,
                    ident_token,
                    block,
                )
                .into(),
            ),
            ItemPath::AssociatedItem(_) | ItemPath::TypeVariant(_, _) => None,
            ItemPath::ImplBlock(_) => todo!(),
            ItemPath::Attr(_, _) => todo!(),
        }
    }

    pub fn syn_node_path(self, db: &::salsa::Db) -> ItemSynNodePath {
        match self {
            ItemSynNode::Submodule(node) => node.syn_node_path(db).into(),
            ItemSynNode::MajorItem(node) => node.syn_node_path(db).into(),
            ItemSynNode::AssociatedItem(node) => node.syn_node_path(db).into(),
            ItemSynNode::TypeVariant(node) => node.syn_node_path(db).into(),
            ItemSynNode::ImplBlock(node) => node.syn_node_path(db).into(),
        }
    }

    pub fn ast_idx(self, db: &::salsa::Db) -> AstIdx {
        match self {
            ItemSynNode::Submodule(node) => node.ast_idx(db),
            ItemSynNode::MajorItem(node) => node.ast_idx(db),
            ItemSynNode::AssociatedItem(_) => todo!(),
            ItemSynNode::TypeVariant(_) => todo!(),
            ItemSynNode::ImplBlock(_) => todo!(),
        }
    }

    pub fn ident_token(self, db: &::salsa::Db) -> IdentToken {
        match self {
            ItemSynNode::Submodule(symbol) => symbol.ident_token(db),
            ItemSynNode::MajorItem(symbol) => symbol.ident_token(db),
            ItemSynNode::AssociatedItem(_) => todo!(),
            ItemSynNode::TypeVariant(_) => todo!(),
            ItemSynNode::ImplBlock(_) => todo!(),
        }
    }
}

pub trait HasAssociatedItemPaths: Copy {
    type AssociatedItemPath;

    fn associated_item_paths(self, db: &::salsa::Db) -> &[(Ident, Self::AssociatedItemPath)];
}

pub trait HasAttrPaths: Copy {
    type AttrPath;

    fn attr_paths(self, db: &::salsa::Db) -> &[Self::AttrPath];
}
