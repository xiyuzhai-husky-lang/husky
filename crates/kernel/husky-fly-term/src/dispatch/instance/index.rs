mod ethereal;
mod hollow;
mod solid;

use self::ethereal::*;
use super::*;
use assoc_item::trai_for_ty_item::index::FlyIndexSignature;

pub type FlyIndexDynamicDispatch = FlyInstanceDispatch<FlyIndexSignature>;

impl FlyTerm {
    pub fn dispatch_index(
        self,
        engine: &mut impl FlyTermEngineMut,
        expr_idx: SynExprIdx,
        index_ty: FlyTerm,
    ) -> FlyTermMaybeResult<FlyIndexDynamicDispatch> {
        self.dispatch_index_aux(
            engine,
            expr_idx,
            index_ty,
            FlyIndirections::new(self.initial_place()),
        )
    }

    fn dispatch_index_aux(
        self,
        engine: &mut impl FlyTermEngineMut,
        expr_idx: SynExprIdx,
        index_ty: FlyTerm,
        indirections: FlyIndirections,
    ) -> FlyTermMaybeResult<FlyIndexDynamicDispatch> {
        match self.base_resolved(engine) {
            FlyTermBase::Eth(owner_ty) => {
                ethereal_owner_ty_index_dispatch(engine, expr_idx, owner_ty, index_ty, indirections)
            }
            FlyTermBase::Sol(owner_ty) => owner_ty.disambiguate_index(engine, expr_idx, index_ty),
            FlyTermBase::Hol(_) => todo!(),
            FlyTermBase::Place => todo!(),
        }
    }
}
