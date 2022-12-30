mod form;
mod trai;
mod trai_item;
mod ty;
mod ty_item;
mod variant;

use crate::*;

pub use form::*;
pub use trai::*;
pub use trai_item::*;
pub use ty::*;
pub use ty_item::*;
pub use variant::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Decl {
    Type(TypeDecl),
    Form(FormDecl),
    Trait(TraitDecl),
    TypeItem(TypeItemDecl),
    TraitItem(TraitItemDecl),
    Variant(VariantDecl),
}

impl Decl {
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            Decl::Type(decl) => decl.ast_idx(db),
            Decl::Form(decl) => decl.ast_idx(db),
            Decl::Trait(decl) => decl.ast_idx(db),
            Decl::TypeItem(decl) => decl.ast_idx(db),
            Decl::TraitItem(decl) => decl.ast_idx(db),
            Decl::Variant(_) => todo!(),
        }
    }
}

impl From<TraitDecl> for Decl {
    fn from(v: TraitDecl) -> Self {
        Self::Trait(v)
    }
}

impl From<FormDecl> for Decl {
    fn from(v: FormDecl) -> Self {
        Self::Form(v)
    }
}

impl From<TypeDecl> for Decl {
    fn from(v: TypeDecl) -> Self {
        Self::Type(v)
    }
}

impl From<TraitItemDecl> for Decl {
    fn from(v: TraitItemDecl) -> Self {
        Self::TraitItem(v)
    }
}

impl From<TypeItemDecl> for Decl {
    fn from(v: TypeItemDecl) -> Self {
        Self::TypeItem(v)
    }
}
