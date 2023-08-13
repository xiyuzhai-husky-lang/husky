mod ethereal;
mod hollow;
mod solid;

use self::ethereal::*;
use self::hollow::*;
use self::solid::*;
use super::*;

pub type FluffyIndexDispatch = FluffyDynamicDispatch<FluffyIndexSignature>;

impl FluffyTerm {
    pub fn dispatch_index(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        index_ty: FluffyTerm,
    ) -> FluffyTermMaybeResult<FluffyIndexDispatch> {
        match self.base_resolved(engine) {
            FluffyTermBase::Ethereal(owner_ty) => {
                ethereal_owner_ty_index_dispatch(engine, expr_idx, owner_ty, index_ty)
            }
            FluffyTermBase::Solid(owner_ty) => {
                owner_ty.disambiguate_index(engine, expr_idx, index_ty)
            }
            FluffyTermBase::Hollow(_) => todo!(),
        }
    }
}
