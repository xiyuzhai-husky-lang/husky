use super::*;
use crate::quary::FlyQuary;
use path::assoc_item::ty_item::TypeItemPath;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SemaBinaryOprFlySignature {
    Builtin,
}

impl IsInstanceItemFlySignature for SemaBinaryOprFlySignature {
    type Path = TraitForTypeItemPath;

    fn expr_ty(&self, self_value_final_quary: FlyQuary) -> FlyTermResult<FlyTerm> {
        todo!()
    }

    fn path(&self) -> Option<Self::Path> {
        // ad hoc
        None
    }

    fn instantiation(&self) -> Option<&FlyInstantiation> {
        todo!()
    }
}
