mod form;
mod trai;
mod trai_item;
mod ty;
mod ty_item;

use crate::*;

pub use form::*;
pub use trai::*;
pub use trai_item::*;
pub use ty::*;
pub use ty_item::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Decl {
    Type(TypeDecl),
    Form(FormDecl),
    Trait(TraitDecl),
}
