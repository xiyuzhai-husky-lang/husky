use super::*;

impl<'a> SemExprBuilder<'a> {
    pub(super) fn calc_binary_comparison_expr_ty(
        &mut self,
        lopd: SynExprIdx,
        opr: BinaryComparisonOpr,
        ropd: SynExprIdx,
    ) -> (
        SemExprIdx,
        SemBinaryOpr,
        SemExprIdx,
        SemExprDataResult<SemaBinaryOprInstanceDispatch>,
        SemExprTypeResult<FlyTerm>,
    ) {
        let (lopd_sem_expr_idx, lopd_ty) = self.build_expr_with_ty(lopd, ExpectAnyOriginal);
        let ropd_sem_expr_idx = match lopd_ty {
            Some(destination) => self.build_expr(ropd, ExpectCoercion::new_pure(self, destination)),
            None => self.build_expr(ropd, ExpectAnyDerived),
        };
        (
            lopd_sem_expr_idx,
            SemBinaryOpr::Comparison(opr),
            ropd_sem_expr_idx,
            Ok(SemaBinaryOprInstanceDispatch::builtin()),
            Ok(self.term_menu().bool_ty_ontology().into()),
        )
    }
}
