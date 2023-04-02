mod associated_item;
mod form;
mod impl_block;
mod trai;
mod ty;
mod variant;

pub use associated_item::*;
pub use form::*;
pub use impl_block::*;
pub use trai::*;
pub use ty::*;
pub use variant::*;

use crate::*;
use parsec::{parse_separated_list, HasParseState};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum Decl {
    Type(TypeDecl),
    Form(FormDecl),
    Trait(TraitDecl),
    Impl(ImplBlockDecl),
    AssociatedItem(AssociatedItemDecl),
    Variant(VariantDecl),
}

impl Decl {
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            Decl::Type(decl) => decl.ast_idx(db),
            Decl::Form(decl) => decl.ast_idx(db),
            Decl::Trait(decl) => decl.ast_idx(db),
            Decl::Impl(decl) => decl.ast_idx(db),
            Decl::AssociatedItem(decl) => decl.ast_idx(db),
            Decl::Variant(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ImplicitParameterDecl] {
        match self {
            Decl::Type(decl) => decl.implicit_parameters(db),
            Decl::Form(decl) => decl.implicit_parameters(db),
            Decl::Trait(decl) => decl.implicit_parameters(db),
            Decl::Impl(decl) => decl.implicit_parameters(db),
            Decl::AssociatedItem(decl) => decl.implicit_parameters(db),
            Decl::Variant(_decl) => &[],
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> ExprRegion {
        match self {
            Decl::Type(decl) => decl.expr_region(db).into(),
            Decl::Form(decl) => decl.expr_region(db).into(),
            Decl::Trait(decl) => decl.expr_region(db).into(),
            Decl::Impl(decl) => decl.expr_region(db).into(),
            Decl::AssociatedItem(decl) => decl.expr_region(db).into(),
            Decl::Variant(_decl) => todo!(),
        }
    }

    pub fn path(self, db: &dyn DeclDb) -> Option<EntityPath> {
        match self {
            Decl::Type(decl) => Some(decl.path(db).into()),
            Decl::Form(decl) => Some(decl.path(db).into()),
            Decl::Trait(decl) => Some(decl.path(db).into()),
            Decl::Impl(_decl) => None,
            Decl::AssociatedItem(decl) => decl.path(db).map(|path| path.into()),
            Decl::Variant(_decl) => todo!(),
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
        match self {
            EntityPath::Module(_) => todo!(),
            EntityPath::ModuleItem(path) => path.decl(db), // change this to trait method
            EntityPath::AssociatedItem(_) => todo!(),
            EntityPath::TypeVariant(_) => todo!(),
        }
    }
}

impl HasDecl for ModuleItemPath {
    type Decl = Decl;

    fn decl<'a>(self, db: &'a dyn DeclDb) -> DeclResultRef<'a, Self::Decl> {
        match self {
            ModuleItemPath::Type(path) => path.decl(db).map(Into::into),
            ModuleItemPath::Trait(path) => path.decl(db).map(Into::into),
            ModuleItemPath::Form(path) => path.decl(db).map(Into::into),
        }
    }
}
