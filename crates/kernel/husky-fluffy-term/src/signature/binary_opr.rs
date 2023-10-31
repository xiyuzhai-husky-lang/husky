use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::debug_with_db(db = FluffyTermDb, jar = FluffyTermJar)]
pub enum BinaryOprFluffySignature {
    Builtin,
}

impl MemberSignature for BinaryOprFluffySignature {
    fn expr_ty(&self) -> FluffyTermResult<FluffyTerm> {
        todo!()
    }
}
