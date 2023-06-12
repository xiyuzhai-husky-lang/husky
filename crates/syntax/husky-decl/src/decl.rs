mod associated_item;
mod fugitive;
mod impl_block;
mod trai;
mod ty;
mod ty_variant;

pub use self::associated_item::*;
pub use self::fugitive::*;
pub use self::impl_block::*;
pub use self::trai::*;
pub use self::ty::*;
pub use self::ty_variant::*;

use crate::*;
use parsec::{parse_separated_list, HasStreamState};

type SmallVecImpl<T> = smallvec::SmallVec<[T; 2]>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum NodeDecl {
    Type(TypeNodeDecl),
    Fugitive(FugitiveNodeDecl),
    Trait(TraitNodeDecl),
    ImplBlock(ImplBlockNodeDecl),
    AssociatedItem(AssociatedItemNodeDecl),
    TypeVariant(TypeVariantNodeDecl),
}

impl NodeDecl {
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            NodeDecl::Type(decl) => decl.ast_idx(db),
            NodeDecl::Fugitive(decl) => decl.ast_idx(db),
            NodeDecl::Trait(decl) => decl.ast_idx(db),
            NodeDecl::ImplBlock(decl) => decl.ast_idx(db),
            NodeDecl::AssociatedItem(decl) => decl.ast_idx(db),
            NodeDecl::TypeVariant(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ImplicitParameterDeclPattern] {
        match self {
            NodeDecl::Type(decl) => decl.implicit_parameters(db),
            NodeDecl::Fugitive(decl) => decl.implicit_parameters(db),
            NodeDecl::Trait(decl) => decl.implicit_parameters(db),
            NodeDecl::ImplBlock(decl) => decl.implicit_parameters(db),
            NodeDecl::AssociatedItem(decl) => decl.implicit_parameters(db),
            NodeDecl::TypeVariant(_decl) => &[],
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> ExprRegion {
        match self {
            NodeDecl::Type(decl) => decl.expr_region(db).into(),
            NodeDecl::Fugitive(decl) => decl.expr_region(db).into(),
            NodeDecl::Trait(decl) => decl.expr_region(db).into(),
            NodeDecl::ImplBlock(decl) => decl.expr_region(db).into(),
            NodeDecl::AssociatedItem(decl) => decl.expr_region(db).into(),
            NodeDecl::TypeVariant(_decl) => todo!(),
        }
    }

    pub fn node_path(self, db: &dyn DeclDb) -> EntityNodePath {
        match self {
            NodeDecl::Type(decl) => decl.node_path(db).into(),
            NodeDecl::Fugitive(decl) => decl.node_path(db).into(),
            NodeDecl::Trait(decl) => decl.node_path(db).into(),
            NodeDecl::ImplBlock(decl) => decl.node_path(db).into(),
            NodeDecl::AssociatedItem(decl) => decl.node_path(db).into(),
            NodeDecl::TypeVariant(decl) => decl.node_path(db).into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum Decl {
    Type(TypeDecl),
    Fugitive(FugitiveDecl),
    Trait(TraitDecl),
    ImplBlock(ImplBlockDecl),
    AssociatedItem(AssociatedItemDecl),
    TypeVariant(TypeVariantDecl),
}

impl Decl {
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            Decl::Type(decl) => decl.ast_idx(db),
            Decl::Fugitive(decl) => decl.ast_idx(db),
            Decl::Trait(decl) => decl.ast_idx(db),
            Decl::ImplBlock(decl) => decl.ast_idx(db),
            Decl::AssociatedItem(decl) => decl.ast_idx(db),
            Decl::TypeVariant(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ImplicitParameterDeclPattern] {
        match self {
            Decl::Type(decl) => decl.implicit_parameters(db),
            Decl::Fugitive(decl) => decl.implicit_parameters(db),
            Decl::Trait(decl) => decl.implicit_parameters(db),
            Decl::ImplBlock(decl) => decl.implicit_parameters(db),
            Decl::AssociatedItem(decl) => decl.implicit_parameters(db),
            Decl::TypeVariant(_decl) => &[],
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> ExprRegion {
        match self {
            Decl::Type(decl) => decl.expr_region(db).into(),
            Decl::Fugitive(decl) => decl.expr_region(db).into(),
            Decl::Trait(decl) => decl.expr_region(db).into(),
            Decl::ImplBlock(decl) => decl.expr_region(db).into(),
            Decl::AssociatedItem(decl) => decl.expr_region(db).into(),
            Decl::TypeVariant(_decl) => todo!(),
        }
    }

    pub fn node_path(self, db: &dyn DeclDb) -> EntityNodePath {
        match self {
            Decl::Type(decl) => decl.node_path(db).into(),
            Decl::Fugitive(decl) => decl.node_path(db).into(),
            Decl::Trait(decl) => decl.node_path(db).into(),
            Decl::ImplBlock(decl) => decl.node_path(db).into(),
            Decl::AssociatedItem(decl) => decl.node_path(db).into(),
            Decl::TypeVariant(decl) => decl.node_path(db).into(),
        }
    }
}

pub trait HasDecl: Copy {
    type Decl;

    fn decl<'a>(self, db: &'a dyn DeclDb) -> DeclResultRef<'a, Self::Decl>;
}

impl HasDecl for EntityPath {
    type Decl = Decl;

    fn decl<'a>(self, db: &'a dyn DeclDb) -> DeclResultRef<'a, Self::Decl> {
        self.node_path(db).decl(db)
    }
}

impl HasDecl for EntityNodePath {
    type Decl = Decl;

    fn decl<'a>(self, db: &'a dyn DeclDb) -> DeclResultRef<'a, Self::Decl> {
        match self {
            EntityNodePath::ModuleItem(id) => id.decl(db), // change this to trait method
            EntityNodePath::TypeVariant(_) => todo!(),
            EntityNodePath::ImplBlock(_) => todo!(),
            EntityNodePath::AssociatedItem(_) => todo!(),
        }
    }
}

impl HasDecl for ModuleItemNodePath {
    type Decl = Decl;

    fn decl<'a>(self, db: &'a dyn DeclDb) -> DeclResultRef<'a, Self::Decl> {
        match self {
            ModuleItemNodePath::Type(id) => id.decl(db).map(Into::into),
            ModuleItemNodePath::Trait(id) => id.decl(db).map(Into::into),
            ModuleItemNodePath::Fugitive(id) => id.decl(db).map(Into::into),
        }
    }
}
