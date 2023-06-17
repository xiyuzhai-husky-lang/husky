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
    ModuleItem(ModuleItemNodePath),
    TypeVariant(TypeVariantNodePath),
    ImplBlock(ImplBlockNodePath),
    AssociatedItem(AssociatedItemNodePath),
}

impl EntityNodePath {
    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        match self {
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
            EntityPath::Module(module_path) => {
                Some(SubmoduleNode::new(db, module_path, visibility, ast_idx, ident_token).into())
            }
            EntityPath::ModuleItem(module_item_path) => Some(
                ModuleItemNode::new(
                    db,
                    ModuleItemNodePath::new(db, registry, module_item_path),
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

    pub fn ast_idx(self, db: &dyn EntityTreeDb) -> AstIdx {
        match self {
            EntityNode::Submodule(symbol) => symbol.ast_idx(db),
            EntityNode::ModuleItem(symbol) => symbol.ast_idx(db),
        }
    }

    pub fn ident_token(self, db: &dyn EntityTreeDb) -> IdentToken {
        match self {
            EntityNode::Submodule(symbol) => symbol.ident_token(db),
            EntityNode::ModuleItem(symbol) => symbol.ident_token(db),
        }
    }
}
