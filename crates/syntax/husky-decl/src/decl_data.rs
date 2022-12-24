mod form;
mod trai;
mod ty;

use crate::*;

pub use form::*;
pub use trai::*;
pub use ty::*;

#[derive(Debug, PartialEq, Eq)]
pub enum DeclData {
    Form(FormDecl),
}
