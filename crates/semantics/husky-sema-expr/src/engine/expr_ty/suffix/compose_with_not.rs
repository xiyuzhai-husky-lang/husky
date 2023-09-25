use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_compose_with_not_expr_ty(
        &mut self,
        opd: SynExprIdx,
        final_destination: FinalDestination,
    ) -> SemaExprResult<(SynExprDisambiguation, SemaExprResult<FluffyTerm>)> {
        todo!()
    }
}
