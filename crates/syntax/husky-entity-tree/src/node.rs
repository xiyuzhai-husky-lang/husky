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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
#[enum_class::from_variants]
pub enum EntityNodePath {
    Submodule(SubmoduleNodePath),
    ModuleItem(ModuleItemNodePath),
    TypeVariant(TypeVariantNodePath),
    ImplBlock(ImplBlockNodePath),
    AssociatedItem(AssociatedItemNodePath),
}

impl EntityNodePath {
    pub fn path(self, db: &dyn EntityTreeDb) -> Option<EntityPath> {
        match self {
            EntityNodePath::Submodule(node_path) => node_path.path(db).map(Into::into),
            EntityNodePath::ModuleItem(node_path) => node_path.path(db).map(Into::into),
            EntityNodePath::TypeVariant(node_path) => node_path.path(db).map(Into::into),
            EntityNodePath::ImplBlock(node_path) => node_path.path(db).map(Into::into),
            EntityNodePath::AssociatedItem(node_path) => node_path.path(db).map(Into::into),
        }
    }

    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        match self {
            EntityNodePath::Submodule(node_path) => node_path.module_path(db),
            EntityNodePath::ModuleItem(node_path) => node_path.module_path(db),
            EntityNodePath::TypeVariant(node_path) => node_path.module_path(db),
            EntityNodePath::ImplBlock(node_path) => node_path.module_path(db),
            EntityNodePath::AssociatedItem(node_path) => node_path.module_path(db),
        }
    }

    pub fn toolchain(self, db: &dyn EntityTreeDb) -> Toolchain {
        self.module_path(db).toolchain(db)
    }
}

pub trait HasNodePath: Copy {
    type NodePath;

    fn node_path(self, db: &dyn EntityTreeDb) -> Self::NodePath;
}

impl HasNodePath for EntityPath {
    type NodePath = EntityNodePath;

    fn node_path(self, db: &dyn EntityTreeDb) -> Self::NodePath {
        match self {
            EntityPath::Module(path) => todo!(),
            EntityPath::ModuleItem(_) => todo!(),
            EntityPath::AssociatedItem(_) => todo!(),
            EntityPath::TypeVariant(_) => todo!(),
            EntityPath::ImplBlock(_) => todo!(),
        }
    }
}

#[derive(Default)]
pub(crate) struct EntityNodeRegistry {
    next_disambiguators: VecPairMap<EntityPath, u8>,
    errors: Vec<EntityTreeError>,
}

impl EntityNodeRegistry {
    pub(crate) fn finish_with_errors(self) -> Vec<EntityTreeError> {
        self.errors
    }

    fn issue_maybe_ambiguous<P: Copy + Into<EntityPath>>(
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
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
#[enum_class::from_variants]
pub enum EntityNode {
    Submodule(SubmoduleNode),
    ModuleItem(ModuleItemNode),
    AssociatedItem(AssociatedItemNode),
    TypeVariant(TypeVariantNode),
    ImplBlock(ImplBlockNode),
}

impl EntityNode {
    pub(crate) fn try_new(
        db: &dyn EntityTreeDb,
        registry: &mut EntityNodeRegistry,
        visibility: Scope,
        ast_idx: AstIdx,
        ident_token: IdentToken,
        entity_path: EntityPath,
    ) -> Option<Self> {
        match entity_path {
            EntityPath::Module(submodule_path) => Some(
                SubmoduleNode::new(
                    db,
                    registry,
                    submodule_path,
                    visibility,
                    ast_idx,
                    ident_token,
                )
                .into(),
            ),
            EntityPath::ModuleItem(module_item_path) => Some(
                ModuleItemNode::new(
                    db,
                    registry,
                    module_item_path,
                    visibility,
                    ast_idx,
                    ident_token,
                )
                .into(),
            ),
            EntityPath::AssociatedItem(_) | EntityPath::TypeVariant(_) => None,
            EntityPath::ImplBlock(_) => todo!(),
        }
    }

    pub fn node_path(self, db: &dyn EntityTreeDb) -> EntityNodePath {
        match self {
            EntityNode::Submodule(node) => node.node_path(db).into(),
            EntityNode::ModuleItem(node) => node.node_path(db).into(),
            EntityNode::AssociatedItem(node) => node.node_path(db).into(),
            EntityNode::TypeVariant(node) => node.node_path(db).into(),
            EntityNode::ImplBlock(node) => node.node_path(db).into(),
        }
    }

    pub fn ast_idx(self, db: &dyn EntityTreeDb) -> AstIdx {
        match self {
            EntityNode::Submodule(node) => node.ast_idx(db),
            EntityNode::ModuleItem(node) => node.ast_idx(db),
            EntityNode::AssociatedItem(_) => todo!(),
            EntityNode::TypeVariant(_) => todo!(),
            EntityNode::ImplBlock(_) => todo!(),
        }
    }

    pub fn ident_token(self, db: &dyn EntityTreeDb) -> IdentToken {
        match self {
            EntityNode::Submodule(symbol) => symbol.ident_token(db),
            EntityNode::ModuleItem(symbol) => symbol.ident_token(db),
            EntityNode::AssociatedItem(_) => todo!(),
            EntityNode::TypeVariant(_) => todo!(),
            EntityNode::ImplBlock(_) => todo!(),
        }
    }
}

// #[inline(always)]
// fn resolve_module_item_symbol(&self, id: ModuleItemNodePath) -> ModuleItemNode {
//     todo!()
//     //         let db = self.db;
//     //         let path = id.path(db);
//     //         let ident = path.ident(db);
//     //         let Some(entity_symbol) = self
//     //             .entity_tree_sheet
//     //             .module_symbols()
//     //             .resolve_ident(ident)
//     //             else {
//     //                 use salsa::DisplayWithDb;
//     //                 panic!(r#"
//     //     Path `{}` is invalid!
//     //     This is very likely caused by expect item in standard library.
//     // "#, path.display(db))
//     //             };
//     //         entity_symbol.module_item_node().unwrap()
// }

impl EntityNodePath {
    pub fn node(self, db: &dyn EntityTreeDb) -> EntityNode {
        match self {
            EntityNodePath::Submodule(path) => path.node(db).into(),
            EntityNodePath::ModuleItem(path) => path.node(db).into(),
            EntityNodePath::AssociatedItem(path) => path.node(db).into(),
            EntityNodePath::TypeVariant(path) => path.node(db).into(),
            EntityNodePath::ImplBlock(path) => path.node(db).into(),
        }
    }
}
