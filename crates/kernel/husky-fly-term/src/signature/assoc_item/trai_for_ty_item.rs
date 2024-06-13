pub mod assoc_ritchie;
pub mod assoc_static_var;
pub mod assoc_ty;
pub mod binary_opr;
pub mod index;
pub mod method_ritchie;

use super::*;
use husky_entity_path::path::assoc_item::trai_for_ty_item::TraitForTypeItemPath;

#[derive(Debug, PartialEq, Eq)]
pub enum TraitForTypeItemFlySignature {}

impl TraitForTypeItemFlySignature {
    pub fn ty(&self) -> FlyTerm {
        todo!()
    }
}
