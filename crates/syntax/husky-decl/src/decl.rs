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
use parsec::{parse_separated_list, HasStreamState};

type SmallVecImpl<T> = smallvec::SmallVec<[T; 2]>;

/// A `NodeDecl` is a tolerant information-preserving declaration
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum NodeDecl {
    Submodule(SubmoduleNodeDecl),
    ModuleItem(ModuleItemNodeDecl),
    ImplBlock(ImplBlockNodeDecl),
    AssociatedItem(AssociatedItemNodeDecl),
    TypeVariant(TypeVariantNodeDecl),
}

impl NodeDecl {
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            NodeDecl::Submodule(decl) => decl.ast_idx(db),
            NodeDecl::ModuleItem(decl) => decl.ast_idx(db),
            NodeDecl::ImplBlock(decl) => decl.ast_idx(db),
            NodeDecl::AssociatedItem(decl) => decl.ast_idx(db),
            NodeDecl::TypeVariant(decl) => decl.ast_idx(db),
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> Option<ExprRegion> {
        match self {
            NodeDecl::Submodule(_) => None,
            NodeDecl::ModuleItem(decl) => decl.expr_region(db).into(),
            NodeDecl::ImplBlock(decl) => decl.expr_region(db).into(),
            NodeDecl::AssociatedItem(decl) => decl.expr_region(db).into(),
            NodeDecl::TypeVariant(_decl) => todo!(),
        }
    }

    pub fn node_path(self, db: &dyn DeclDb) -> EntityNodePath {
        match self {
            NodeDecl::Submodule(decl) => decl.node_path(db).into(),
            NodeDecl::ModuleItem(decl) => decl.node_path(db).into(),
            NodeDecl::ImplBlock(decl) => decl.node_path(db).into(),
            NodeDecl::AssociatedItem(decl) => decl.node_path(db).into(),
            NodeDecl::TypeVariant(decl) => decl.node_path(db).into(),
        }
    }
}

/// A `Decl` is a strict version, handy for subsequent processing
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum Decl {
    Submodule(SubmoduleDecl),
    ModuleItem(ModuleItemDecl),
    ImplBlock(ImplBlockDecl),
    AssociatedItem(AssociatedItemDecl),
    TypeVariant(TypeVariantDecl),
}

impl Decl {
    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ImplicitParameterDeclPattern] {
        match self {
            Decl::Submodule(_) => todo!(),
            Decl::ModuleItem(decl) => decl.implicit_parameters(db),
            Decl::ImplBlock(decl) => decl.implicit_parameters(db),
            Decl::AssociatedItem(decl) => decl.implicit_parameters(db),
            Decl::TypeVariant(_decl) => &[],
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> Option<ExprRegion> {
        match self {
            Decl::Submodule(_) => None,
            Decl::ModuleItem(decl) => decl.expr_region(db).into(),
            Decl::ImplBlock(decl) => decl.expr_region(db).into(),
            Decl::AssociatedItem(decl) => decl.expr_region(db).into(),
            Decl::TypeVariant(_decl) => todo!(),
        }
    }

    pub fn path(self, db: &dyn DeclDb) -> EntityPath {
        match self {
            Decl::Submodule(_) => todo!(),
            Decl::ModuleItem(decl) => decl.path(db).into(),
            Decl::ImplBlock(decl) => decl.path(db).into(),
            Decl::AssociatedItem(decl) => decl.path(db).into(),
            Decl::TypeVariant(decl) => decl.path(db).into(),
        }
    }
}

pub trait HasNodeDecl: Copy {
    type NodeDecl;

    fn node_decl<'a>(self, db: &'a dyn DeclDb) -> Self::NodeDecl;
}

impl HasNodeDecl for EntityNodePath {
    type NodeDecl = NodeDecl;

    fn node_decl<'a>(self, db: &'a dyn DeclDb) -> Self::NodeDecl {
        match self {
            EntityNodePath::ModuleItem(node_path) => node_path.node_decl(db).into(),
            EntityNodePath::TypeVariant(_) => todo!(),
            EntityNodePath::ImplBlock(_) => todo!(),
            EntityNodePath::AssociatedItem(_) => todo!(),
            EntityNodePath::Submodule(node_path) => node_path.node_decl(db).into(),
        }
    }
}

pub trait HasDecl: Copy {
    type Decl;

    fn decl(self, db: &dyn DeclDb) -> DeclResult<Self::Decl>;
}

impl HasDecl for EntityPath {
    type Decl = Decl;

    fn decl(self, db: &dyn DeclDb) -> DeclResult<Self::Decl> {
        match self {
            EntityPath::Module(path) => path.decl(db).map(Into::into),
            EntityPath::ModuleItem(path) => path.decl(db).map(Into::into),
            EntityPath::AssociatedItem(path) => path.decl(db).map(Into::into),
            EntityPath::TypeVariant(path) => path.decl(db).map(Into::into),
            EntityPath::ImplBlock(path) => path.decl(db).map(Into::into),
        }
    }
}
