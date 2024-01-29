use super::*;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SemaBinaryOprFluffySignature {
    Builtin,
}

impl MemberSignature for SemaBinaryOprFluffySignature {
    fn expr_ty(&self, self_value_final_place: FluffyPlace) -> FluffyTermResult<FluffyTerm> {
        todo!()
    }
}
