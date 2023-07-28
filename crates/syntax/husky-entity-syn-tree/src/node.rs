mod associated_item;
mod impl_block;
mod module_item;
mod submodule;
mod ty_variant;

pub use self::associated_item::*;
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
pub enum EntitySynNodePath {
    Submodule(SubmoduleSynNodePath),
    ModuleItem(ModuleItemSynNodePath),
    TypeVariant(TypeVariantSynNodePath),
    ImplBlock(ImplBlockSynNodePath),
    AssociatedItem(AssociatedItemSynNodePath),
}

impl EntitySynNodePath {
    pub fn path(self, db: &dyn EntitySynTreeDb) -> Option<ItemPath> {
        match self {
            EntitySynNodePath::Submodule(syn_node_path) => syn_node_path.path(db).map(Into::into),
            EntitySynNodePath::ModuleItem(syn_node_path) => syn_node_path.path(db).map(Into::into),
            EntitySynNodePath::TypeVariant(syn_node_path) => syn_node_path.path(db).map(Into::into),
            EntitySynNodePath::ImplBlock(syn_node_path) => syn_node_path.path(db).map(Into::into),
            EntitySynNodePath::AssociatedItem(syn_node_path) => {
                syn_node_path.path(db).map(Into::into)
            }
        }
    }

    pub fn module_path(self, db: &dyn EntitySynTreeDb) -> ModulePath {
        match self {
            EntitySynNodePath::Submodule(syn_node_path) => syn_node_path.module_path(db),
            EntitySynNodePath::ModuleItem(syn_node_path) => syn_node_path.module_path(db),
            EntitySynNodePath::TypeVariant(syn_node_path) => syn_node_path.module_path(db),
            EntitySynNodePath::ImplBlock(syn_node_path) => syn_node_path.module_path(db),
            EntitySynNodePath::AssociatedItem(syn_node_path) => syn_node_path.module_path(db),
        }
    }

    pub fn toolchain(self, db: &dyn EntitySynTreeDb) -> Toolchain {
        self.module_path(db).toolchain(db)
    }
}

pub trait HasSynNodePath: Copy {
    type SynNodePath;

    fn syn_node_path(self, db: &dyn EntitySynTreeDb) -> Self::SynNodePath;
}

impl HasSynNodePath for ItemPath {
    type SynNodePath = EntitySynNodePath;

    fn syn_node_path(self, db: &dyn EntitySynTreeDb) -> Self::SynNodePath {
        match self {
            ItemPath::Submodule(path) => todo!(),
            ItemPath::ModuleItem(_) => todo!(),
            ItemPath::AssociatedItem(_) => todo!(),
            ItemPath::TypeVariant(_) => todo!(),
            ItemPath::ImplBlock(_) => todo!(),
        }
    }
}

#[derive(Default)]
pub(crate) struct EntityNodeRegistry {
    next_disambiguators: VecPairMap<ItemPath, u8>,
    errors: Vec<EntityTreeError>,
}

impl EntityNodeRegistry {
    pub(crate) fn finish_with_errors(self) -> Vec<EntityTreeError> {
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
pub enum EntitySynNode {
    Submodule(SubmoduleSynNode),
    ModuleItem(ModuleItemSynNode),
    AssociatedItem(AssociatedItemSynNode),
    TypeVariant(TypeVariantSynNode),
    ImplBlock(ImplBlockSynNode),
}

impl EntitySynNode {
    pub(crate) fn try_new(
        db: &dyn EntitySynTreeDb,
        registry: &mut EntityNodeRegistry,
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
            ItemPath::ModuleItem(module_item_path) => Some(
                ModuleItemSynNode::new(
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
        }
    }

    pub fn syn_node_path(self, db: &dyn EntitySynTreeDb) -> EntitySynNodePath {
        match self {
            EntitySynNode::Submodule(node) => node.syn_node_path(db).into(),
            EntitySynNode::ModuleItem(node) => node.syn_node_path(db).into(),
            EntitySynNode::AssociatedItem(node) => node.syn_node_path(db).into(),
            EntitySynNode::TypeVariant(node) => node.syn_node_path(db).into(),
            EntitySynNode::ImplBlock(node) => node.syn_node_path(db).into(),
        }
    }

    pub fn ast_idx(self, db: &dyn EntitySynTreeDb) -> AstIdx {
        match self {
            EntitySynNode::Submodule(node) => node.ast_idx(db),
            EntitySynNode::ModuleItem(node) => node.ast_idx(db),
            EntitySynNode::AssociatedItem(_) => todo!(),
            EntitySynNode::TypeVariant(_) => todo!(),
            EntitySynNode::ImplBlock(_) => todo!(),
        }
    }

    pub fn ident_token(self, db: &dyn EntitySynTreeDb) -> IdentToken {
        match self {
            EntitySynNode::Submodule(symbol) => symbol.ident_token(db),
            EntitySynNode::ModuleItem(symbol) => symbol.ident_token(db),
            EntitySynNode::AssociatedItem(_) => todo!(),
            EntitySynNode::TypeVariant(_) => todo!(),
            EntitySynNode::ImplBlock(_) => todo!(),
        }
    }
}

// #[inline(always)]
// fn resolve_module_item_symbol(&self, id: ModuleItemNodePath) -> ModuleItemNode {
//     todo!()
//     //         let db = self.db;
//     //         let path = id.path(db);
//     //         let ident = path.ident(db);
//     //         let Some(item_symbol) = self
//     //             .item_tree_sheet
//     //             .module_symbols()
//     //             .resolve_ident(ident)
//     //             else {
//     //                 use salsa::DisplayWithDb;
//     //                 panic!(r#"
//     //     Path `{}` is invalid!
//     //     This is very likely caused by expect item in standard library.
//     // "#, path.display(db))
//     //             };
//     //         item_symbol.module_item_node().unwrap()
// }

impl EntitySynNodePath {
    pub fn node(self, db: &dyn EntitySynTreeDb) -> EntitySynNode {
        match self {
            EntitySynNodePath::Submodule(path) => path.node(db).into(),
            EntitySynNodePath::ModuleItem(path) => path.node(db).into(),
            EntitySynNodePath::AssociatedItem(path) => path.node(db).into(),
            EntitySynNodePath::TypeVariant(path) => path.syn_node(db).into(),
            EntitySynNodePath::ImplBlock(path) => path.node(db).into(),
        }
    }
}

pub trait HasItemPaths: Copy {
    type ItemPath;

    fn item_paths(self, db: &dyn EntitySynTreeDb) -> &[(Ident, Self::ItemPath)];
}
