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
#[salsa::debug_with_db(db = SynDeclDb)]
#[enum_class::from_variants]
pub enum SynNodeDecl {
    Submodule(SubmoduleSynNodeDecl),
    ModuleItem(ModuleItemSynNodeDecl),
    ImplBlock(ImplBlockSynNodeDecl),
    AssociatedItem(AssociatedItemSynNodeDecl),
    TypeVariant(TypeVariantSynNodeDecl),
}

impl SynNodeDecl {
    pub fn ast_idx(self, db: &dyn SynDeclDb) -> AstIdx {
        match self {
            SynNodeDecl::Submodule(syn_node_decl) => syn_node_decl.ast_idx(db),
            SynNodeDecl::ModuleItem(syn_node_decl) => syn_node_decl.ast_idx(db),
            SynNodeDecl::ImplBlock(syn_node_decl) => syn_node_decl.ast_idx(db),
            SynNodeDecl::AssociatedItem(syn_node_decl) => syn_node_decl.ast_idx(db),
            SynNodeDecl::TypeVariant(syn_node_decl) => syn_node_decl.ast_idx(db),
        }
    }

    pub fn syn_expr_region(self, db: &dyn SynDeclDb) -> Option<SynExprRegion> {
        match self {
            SynNodeDecl::Submodule(_) => None,
            SynNodeDecl::ModuleItem(syn_node_decl) => syn_node_decl.syn_expr_region(db).into(),
            SynNodeDecl::ImplBlock(syn_node_decl) => syn_node_decl.syn_expr_region(db).into(),
            SynNodeDecl::AssociatedItem(syn_node_decl) => syn_node_decl.syn_expr_region(db).into(),
            SynNodeDecl::TypeVariant(_node_decl) => todo!(),
        }
    }

    pub fn syn_node_path(self, db: &dyn SynDeclDb) -> EntitySynNodePath {
        match self {
            SynNodeDecl::Submodule(syn_node_decl) => syn_node_decl.syn_node_path(db).into(),
            SynNodeDecl::ModuleItem(syn_node_decl) => syn_node_decl.syn_node_path(db).into(),
            SynNodeDecl::ImplBlock(syn_node_decl) => syn_node_decl.syn_node_path(db).into(),
            SynNodeDecl::AssociatedItem(syn_node_decl) => syn_node_decl.syn_node_path(db).into(),
            SynNodeDecl::TypeVariant(syn_node_decl) => syn_node_decl.syn_node_path(db).into(),
        }
    }

    pub fn errors(self, db: &dyn SynDeclDb) -> NodeDeclErrorRefs {
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
#[salsa::debug_with_db(db = SynDeclDb)]
#[enum_class::from_variants]
pub enum Decl {
    Submodule(SubmoduleSynDecl),
    ModuleItem(ModuleItemSynDecl),
    ImplBlock(ImplBlockSynDecl),
    AssociatedItem(AssociatedItemSynDecl),
    TypeVariant(TypeVariantSynDecl),
}

impl Decl {
    pub fn template_parameters<'a>(self, db: &'a dyn SynDeclDb) -> &'a [TemplateParameterDecl] {
        match self {
            Decl::Submodule(_) => todo!(),
            Decl::ModuleItem(decl) => decl.template_parameters(db),
            Decl::ImplBlock(decl) => decl.template_parameters(db),
            Decl::AssociatedItem(decl) => decl.template_parameters(db),
            Decl::TypeVariant(_decl) => &[],
        }
    }

    pub fn syn_expr_region(self, db: &dyn SynDeclDb) -> Option<SynExprRegion> {
        match self {
            Decl::Submodule(_) => None,
            Decl::ModuleItem(decl) => decl.syn_expr_region(db).into(),
            Decl::ImplBlock(decl) => decl.syn_expr_region(db).into(),
            Decl::AssociatedItem(decl) => decl.syn_expr_region(db).into(),
            Decl::TypeVariant(_decl) => todo!(),
        }
    }

    pub fn path(self, db: &dyn SynDeclDb) -> ItemPath {
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

    fn syn_node_decl<'a>(self, db: &'a dyn SynDeclDb) -> Self::NodeDecl;
}

impl HasNodeDecl for EntitySynNodePath {
    type NodeDecl = SynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a dyn SynDeclDb) -> Self::NodeDecl {
        match self {
            EntitySynNodePath::ModuleItem(syn_node_path) => syn_node_path.syn_node_decl(db).into(),
            EntitySynNodePath::TypeVariant(_) => todo!(),
            EntitySynNodePath::ImplBlock(_) => todo!(),
            EntitySynNodePath::AssociatedItem(_) => todo!(),
            EntitySynNodePath::Submodule(syn_node_path) => syn_node_path.syn_node_decl(db).into(),
        }
    }
}

pub trait HasSynDecl: Copy {
    type Decl;

    fn syn_decl(self, db: &dyn SynDeclDb) -> DeclResult<Self::Decl>;
}

impl HasSynDecl for ItemPath {
    type Decl = Decl;

    fn syn_decl(self, db: &dyn SynDeclDb) -> DeclResult<Self::Decl> {
        match self {
            ItemPath::Submodule(path) => path.syn_decl(db).map(Into::into),
            ItemPath::ModuleItem(path) => path.syn_decl(db).map(Into::into),
            ItemPath::AssociatedItem(path) => path.syn_decl(db).map(Into::into),
            ItemPath::TypeVariant(path) => path.syn_decl(db).map(Into::into),
            ItemPath::ImplBlock(path) => path.syn_decl(db).map(Into::into),
        }
    }
}
