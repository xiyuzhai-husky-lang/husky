use super::*;

impl<'a> SemExprBuilder<'a> {
    pub(super) fn calc_compose_with_not_expr_ty(
        &mut self,
        opd: SynExprIdx,
        final_destination: FinalDestination,
    ) -> (SemExprDataResult<SemExprData>, SemExprTypeResult<FlyTerm>) {
        todo!()
    }

    pub(super) fn calc_compose_with_not_expr_ty_given_opd_ty(
        &mut self,
        opd_ty: FlyTerm,
        // final_destination: FinalDestination,
    ) -> (SemExprDataResult<SemaSuffixOpr>, SemExprTypeResult<FlyTerm>) {
        todo!()
    }
}
