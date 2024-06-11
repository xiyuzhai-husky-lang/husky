use super::*;
use either::*;
use husky_fly_term::expectation::sort_or_trai::{ExpectSortOrTrait, ExpectSortOrTraitOutcome};
use husky_regional_token::IdentRegionalToken;
use maybe_result::*;

impl<'a> SemExprBuilder<'a> {
    pub(super) fn calc_ty_as_target_item_ty(
        &mut self,
        syn_expr_idx: SynExprIdx,
        ty: SynExprIdx,
        target: SynExprIdx,
        ident: Ident,
        ident_regional_token_idx: RegionalTokenIdx,
    ) -> (
        SemExprDataResult<StaticDispatch>,
        SemExprTypeResult<FlyTerm>,
    ) {
        let ty = self.build_sem_expr(ty, ExpectSort::ANY);
        let (target, outcome) = self.build_sem_expr_with_outcome(target, ExpectSortOrTrait);
        let Some(outcome) = outcome else { todo!() };
        let Some(ty) = self.infer_expr_term(ty) else {
            todo!()
        };
        let Some(target) = self.infer_expr_term(target) else {
            todo!()
        };
        match outcome {
            ExpectSortOrTraitOutcome::Sort => todo!(),
            ExpectSortOrTraitOutcome::Trait => self.calc_ty_as_trai_item_ty(
                syn_expr_idx,
                ty,
                target,
                ident,
                ident_regional_token_idx,
            ),
        }
    }

    fn calc_ty_as_trai_item_ty(
        &mut self,
        syn_expr_idx: SynExprIdx,
        ty: FlyTerm,
        target: FlyTerm,
        ident: Ident,
        ident_regional_token_idx: RegionalTokenIdx,
    ) -> (
        SemExprDataResult<StaticDispatch>,
        SemExprTypeResult<FlyTerm>,
    ) {
        todo!()
    }
}
