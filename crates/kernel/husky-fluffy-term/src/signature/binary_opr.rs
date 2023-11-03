use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::debug_with_db(db = FluffyTermDb, jar = FluffyTermJar)]
pub enum SemaBinaryOprFluffySignature {
    Builtin,
}

impl MemberSignature for SemaBinaryOprFluffySignature {
    fn expr_ty(&self) -> FluffyTermResult<FluffyTerm> {
        todo!()
    }
}
