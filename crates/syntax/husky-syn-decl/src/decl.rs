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
pub enum SynNodeDecl {
    Submodule(SubmoduleNodeDecl),
    ModuleItem(ModuleItemSynNodeDecl),
    ImplBlock(ImplBlockSynNodeDecl),
    AssociatedItem(AssociatedItemSynNodeDecl),
    TypeVariant(TypeVariantNodeDecl),
}

impl SynNodeDecl {
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            SynNodeDecl::Submodule(syn_node_decl) => syn_node_decl.ast_idx(db),
            SynNodeDecl::ModuleItem(syn_node_decl) => syn_node_decl.ast_idx(db),
            SynNodeDecl::ImplBlock(syn_node_decl) => syn_node_decl.ast_idx(db),
            SynNodeDecl::AssociatedItem(syn_node_decl) => syn_node_decl.ast_idx(db),
            SynNodeDecl::TypeVariant(syn_node_decl) => syn_node_decl.ast_idx(db),
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> Option<SynExprRegion> {
        match self {
            SynNodeDecl::Submodule(_) => None,
            SynNodeDecl::ModuleItem(syn_node_decl) => syn_node_decl.expr_region(db).into(),
            SynNodeDecl::ImplBlock(syn_node_decl) => syn_node_decl.expr_region(db).into(),
            SynNodeDecl::AssociatedItem(syn_node_decl) => syn_node_decl.expr_region(db).into(),
            SynNodeDecl::TypeVariant(_node_decl) => todo!(),
        }
    }

    pub fn syn_node_path(self, db: &dyn DeclDb) -> EntitySynNodePath {
        match self {
            SynNodeDecl::Submodule(syn_node_decl) => syn_node_decl.syn_node_path(db).into(),
            SynNodeDecl::ModuleItem(syn_node_decl) => syn_node_decl.syn_node_path(db).into(),
            SynNodeDecl::ImplBlock(syn_node_decl) => syn_node_decl.syn_node_path(db).into(),
            SynNodeDecl::AssociatedItem(syn_node_decl) => syn_node_decl.syn_node_path(db).into(),
            SynNodeDecl::TypeVariant(syn_node_decl) => syn_node_decl.syn_node_path(db).into(),
        }
    }

    pub fn errors(self, db: &dyn DeclDb) -> NodeDeclErrorRefs {
        match self {
            SynNodeDecl::Submodule(syn_node_decl) => syn_node_decl.errors(db),
            SynNodeDecl::ModuleItem(syn_node_decl) => syn_node_decl.errors(db),
            SynNodeDecl::ImplBlock(syn_node_decl) => syn_node_decl.errors(db),
            SynNodeDecl::AssociatedItem(syn_node_decl) => syn_node_decl.errors(db),
            SynNodeDecl::TypeVariant(syn_node_decl) => syn_node_decl.errors(db),
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
    ImplBlock(ImplBlockSynDecl),
    AssociatedItem(AssociatedItemSynDecl),
    TypeVariant(TypeVariantDecl),
}

impl Decl {
    pub fn generic_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [GenericParameterDecl] {
        match self {
            Decl::Submodule(_) => todo!(),
            Decl::ModuleItem(decl) => decl.generic_parameters(db),
            Decl::ImplBlock(decl) => decl.generic_parameters(db),
            Decl::AssociatedItem(decl) => decl.generic_parameters(db),
            Decl::TypeVariant(_decl) => &[],
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> Option<SynExprRegion> {
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

    fn syn_node_decl<'a>(self, db: &'a dyn DeclDb) -> Self::NodeDecl;
}

impl HasNodeDecl for EntitySynNodePath {
    type NodeDecl = SynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a dyn DeclDb) -> Self::NodeDecl {
        match self {
            EntitySynNodePath::ModuleItem(syn_node_path) => syn_node_path.syn_node_decl(db).into(),
            EntitySynNodePath::TypeVariant(_) => todo!(),
            EntitySynNodePath::ImplBlock(_) => todo!(),
            EntitySynNodePath::AssociatedItem(_) => todo!(),
            EntitySynNodePath::Submodule(syn_node_path) => syn_node_path.syn_node_decl(db).into(),
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
