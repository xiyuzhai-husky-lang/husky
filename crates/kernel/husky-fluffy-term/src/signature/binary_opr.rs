use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::debug_with_db]
pub enum SemaBinaryOprFluffySignature {
    Builtin,
}

impl MemberSignature for SemaBinaryOprFluffySignature {
    fn expr_ty(&self, self_value_final_place: FluffyPlace) -> FluffyTermResult<FluffyTerm> {
        todo!()
    }
}
