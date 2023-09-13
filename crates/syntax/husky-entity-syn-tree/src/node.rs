mod associated_item;
mod decr;
mod impl_block;
mod module_item;
mod submodule;
mod ty_variant;

pub use self::associated_item::*;
pub use self::decr::*;
pub use self::impl_block::*;
pub use self::module_item::*;
pub use self::submodule::*;
pub use self::ty_variant::*;

use crate::*;
use husky_token::IdentToken;
use vec_like::VecPairMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = EntitySynTreeDb)]
#[enum_class::from_variants]
pub enum ItemSynNodePath {
    Submodule(SubmoduleSynNodePath),
    MajorItem(MajorItemSynNodePath),
    TypeVariant(TypeVariantSynNodePath),
    ImplBlock(ImplBlockSynNodePath),
    AssociatedItem(AssociatedItemSynNodePath),
    Decr(DecrSynNodePath),
}

impl<Db> HasModulePath<Db> for ItemSynNodePath
where
    Db: ?Sized + EntitySynTreeDb,
{
    fn module_path(self, db: &Db) -> ModulePath {
        match self {
            ItemSynNodePath::Submodule(syn_node_path) => syn_node_path.module_path(db),
            ItemSynNodePath::MajorItem(syn_node_path) => syn_node_path.module_path(db),
            ItemSynNodePath::TypeVariant(syn_node_path) => syn_node_path.module_path(db),
            ItemSynNodePath::ImplBlock(syn_node_path) => syn_node_path.module_path(db),
            ItemSynNodePath::AssociatedItem(syn_node_path) => syn_node_path.module_path(db),
            ItemSynNodePath::Decr(syn_node_path) => syn_node_path.module_path(db),
        }
    }
}

impl ItemSynNodePath {
    pub fn path(self, db: &dyn EntitySynTreeDb) -> Option<ItemPath> {
        match self {
            ItemSynNodePath::Submodule(syn_node_path) => syn_node_path.path(db).map(Into::into),
            ItemSynNodePath::MajorItem(syn_node_path) => syn_node_path.path(db).map(Into::into),
            ItemSynNodePath::TypeVariant(syn_node_path) => syn_node_path.path(db).map(Into::into),
            ItemSynNodePath::ImplBlock(syn_node_path) => syn_node_path.path(db).map(Into::into),
            ItemSynNodePath::AssociatedItem(syn_node_path) => {
                syn_node_path.path(db).map(Into::into)
            }
            ItemSynNodePath::Decr(syn_node_path) => syn_node_path.path(db).map(Into::into),
        }
    }

    pub fn toolchain(self, db: &dyn EntitySynTreeDb) -> Toolchain {
        self.module_path(db).toolchain(db)
    }

    pub fn decrs(self, db: &dyn EntitySynTreeDb) -> &[(DecrSynNodePath, DecrSynNode)] {
        // ad hoc
        match self {
            ItemSynNodePath::Submodule(_) => &[],
            ItemSynNodePath::MajorItem(path) => path.decrs(db),
            ItemSynNodePath::TypeVariant(_) => &[],
            ItemSynNodePath::ImplBlock(_) => &[],
            ItemSynNodePath::AssociatedItem(_) => &[],
            ItemSynNodePath::Decr(_) => &[],
        }
    }
}

pub trait HasSynNodePath: Copy {
    type SynNodePath;

    fn syn_node_path(self, db: &dyn EntitySynTreeDb) -> Self::SynNodePath;
}

impl HasSynNodePath for ItemPath {
    type SynNodePath = ItemSynNodePath;

    fn syn_node_path(self, db: &dyn EntitySynTreeDb) -> Self::SynNodePath {
        match self {
            ItemPath::Submodule(path) => path.syn_node_path(db).into(),
            ItemPath::MajorItem(path) => path.syn_node_path(db).into(),
            ItemPath::AssociatedItem(path) => path.syn_node_path(db).into(),
            ItemPath::TypeVariant(path) => path.syn_node_path(db).into(),
            ItemPath::ImplBlock(path) => path.syn_node_path(db).into(),
            ItemPath::Decr(path) => path.syn_node_path(db).into(),
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
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntityTreeJar)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = EntitySynTreeDb)]
#[enum_class::from_variants]
pub enum ItemSynNode {
    Submodule(SubmoduleSynNode),
    MajorItem(MajorItemSynNode),
    AssociatedItem(AssociatedItemSynNode),
    TypeVariant(TypeVariantSynNode),
    ImplBlock(ImplBlockSynNode),
}

impl ItemSynNode {
    pub(crate) fn try_new(
        db: &dyn EntitySynTreeDb,
        registry: &mut ItemSynNodePathRegistry,
        visibility: Scope,
        ast_idx: AstIdx,
        ident_token: IdentToken,
        item_path: ItemPath,
        block: DefnBlock,
    ) -> Option<Self> {
        match item_path {
            ItemPath::Submodule(submodule_path) => Some(
                SubmoduleSynNode::new(
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
                MajorItemSynNode::new(
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
            ItemPath::AssociatedItem(_) | ItemPath::TypeVariant(_) => None,
            ItemPath::ImplBlock(_) => todo!(),
            ItemPath::Decr(_) => todo!(),
        }
    }

    pub fn syn_node_path(self, db: &dyn EntitySynTreeDb) -> ItemSynNodePath {
        match self {
            ItemSynNode::Submodule(node) => node.syn_node_path(db).into(),
            ItemSynNode::MajorItem(node) => node.syn_node_path(db).into(),
            ItemSynNode::AssociatedItem(node) => node.syn_node_path(db).into(),
            ItemSynNode::TypeVariant(node) => node.syn_node_path(db).into(),
            ItemSynNode::ImplBlock(node) => node.syn_node_path(db).into(),
        }
    }

    pub fn ast_idx(self, db: &dyn EntitySynTreeDb) -> AstIdx {
        match self {
            ItemSynNode::Submodule(node) => node.ast_idx(db),
            ItemSynNode::MajorItem(node) => node.ast_idx(db),
            ItemSynNode::AssociatedItem(_) => todo!(),
            ItemSynNode::TypeVariant(_) => todo!(),
            ItemSynNode::ImplBlock(_) => todo!(),
        }
    }

    pub fn ident_token(self, db: &dyn EntitySynTreeDb) -> IdentToken {
        match self {
            ItemSynNode::Submodule(symbol) => symbol.ident_token(db),
            ItemSynNode::MajorItem(symbol) => symbol.ident_token(db),
            ItemSynNode::AssociatedItem(_) => todo!(),
            ItemSynNode::TypeVariant(_) => todo!(),
            ItemSynNode::ImplBlock(_) => todo!(),
        }
    }
}

impl ItemSynNodePath {
    pub(crate) fn syn_node(self, db: &dyn EntitySynTreeDb) -> ItemSynNode {
        match self {
            ItemSynNodePath::Submodule(path) => path.syn_node(db).into(),
            ItemSynNodePath::MajorItem(path) => path.syn_node(db).into(),
            ItemSynNodePath::AssociatedItem(path) => path.syn_node(db).into(),
            ItemSynNodePath::TypeVariant(path) => path.syn_node(db).into(),
            ItemSynNodePath::ImplBlock(path) => path.syn_node(db).into(),
            ItemSynNodePath::Decr(_) => todo!(),
        }
    }
}

pub trait HasAssociatedItemPaths: Copy {
    type AssociatedItemPath;

    fn associated_item_paths(
        self,
        db: &dyn EntitySynTreeDb,
    ) -> &[(Ident, Self::AssociatedItemPath)];
}

pub trait HasDecrPaths: Copy {
    type DecrPath;

    fn decr_paths(self, db: &dyn EntitySynTreeDb) -> &[Self::DecrPath];
}
