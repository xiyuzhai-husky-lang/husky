mod ethereal;
mod hollow;

use self::ethereal::*;
use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FluffyIndexSignature {}

pub type FluffyIndexDisambiguation = FluffyMemberDisambiguation<FluffyIndexSignature>;

impl FluffyTerm {
    pub fn index_disambiguation(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: ExprIdx,
        index_ty: FluffyTerm,
    ) -> FluffyTermMaybeResult<FluffyIndexDisambiguation> {
        match self.nested() {
            NestedFluffyTerm::Ethereal(owner_ty) => {
                ethereal_owner_ty_index_disambiguation(engine, expr_idx, owner_ty, index_ty)
            }
            NestedFluffyTerm::Solid(_) => todo!(),
            NestedFluffyTerm::Hollow(_) => todo!(),
        }
    }
}
