use super::*;

impl<'a> SemaExprBuilder<'a> {
    pub(super) fn calc_binary_comparison_expr_ty(
        &mut self,
        lopd: SynExprIdx,
        opr: BinaryComparisonOpr,
        ropd: SynExprIdx,
    ) -> (
        SemaExprIdx,
        SemaBinaryOpr,
        SemaExprIdx,
        SemaExprDataResult<SemaBinaryOprDynamicDispatch>,
        SemaExprTypeResult<FlyTerm>,
    ) {
        let (lopd_sem_expr_idx, lopd_ty) = self.build_sem_expr_with_ty(lopd, ExpectAnyOriginal);
        let ropd_sem_expr_idx = match lopd_ty {
            Some(destination) => {
                self.build_sem_expr(ropd, ExpectCoercion::new_pure(self, destination))
            }
            None => self.build_sem_expr(ropd, ExpectAnyDerived),
        };
        (
            lopd_sem_expr_idx,
            SemaBinaryOpr::Comparison(opr),
            ropd_sem_expr_idx,
            Ok(SemaBinaryOprDynamicDispatch::builtin()),
            Ok(self.term_menu().bool_ty_ontology().into()),
        )
    }
}
