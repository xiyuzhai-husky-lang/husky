mod form;
mod trai;
mod ty;

use crate::*;

pub use form::*;
pub use trai::*;
pub use ty::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Decl {
    Type(TypeDecl),
    Form(FormDecl),
    Trait(TraitDecl),
}
