pub mod assoc_ritchie;
pub mod assoc_static_var;
pub mod assoc_ty;
pub mod binary_opr;
pub mod index;
pub mod method;

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum TraitForTypeItemFlySignature {}

impl TraitForTypeItemFlySignature {
    pub fn ty(&self) -> FlyTerm {
        todo!()
    }
}
