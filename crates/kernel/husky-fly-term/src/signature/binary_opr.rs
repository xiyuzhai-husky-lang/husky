use self::quary::FlyQuary;
use super::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SemaBinaryOprFlySignature {
    Builtin,
}

impl MemberSignature for SemaBinaryOprFlySignature {
    fn expr_ty(&self, self_value_final_quary: FlyQuary) -> FlyTermResult<FlyTerm> {
        todo!()
    }
}
