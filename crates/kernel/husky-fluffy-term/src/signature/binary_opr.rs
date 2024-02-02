use super::*;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SemaBinaryOprFlySignature {
    Builtin,
}

impl MemberSignature for SemaBinaryOprFlySignature {
    fn expr_ty(&self, self_value_final_place: FlyPlace) -> FlyTermResult<FlyTerm> {
        todo!()
    }
}
