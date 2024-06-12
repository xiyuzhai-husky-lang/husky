use super::*;
use crate::quary::FlyQuary;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SemaBinaryOprFlySignature {
    Builtin,
}

impl IsInstanceItemFlySignature for SemaBinaryOprFlySignature {
    fn expr_ty(&self, self_value_final_quary: FlyQuary) -> FlyTermResult<FlyTerm> {
        todo!()
    }
}
