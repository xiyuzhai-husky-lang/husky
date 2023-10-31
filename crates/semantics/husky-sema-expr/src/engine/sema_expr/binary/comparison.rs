use super::*;

impl<'a> SemaExprEngine<'a> {
    pub(super) fn calc_binary_comparison_expr_ty(
        &mut self,
        lopd: SynExprIdx,
        ropd: SynExprIdx,
    ) -> (
        SemaExprIdx,
        SemaExprIdx,
        SemaExprDataResult<BinaryOprDynamicDispatch>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        let (lopd_sema_expr_idx, lopd_ty) =
            self.build_sema_expr_with_its_ty_returned(lopd, ExpectAnyOriginal);
        let ropd_sema_expr_idx = match lopd_ty {
            Some(destination) => {
                self.build_sema_expr(ropd, ExpectCoersion::new_pure(self, destination))
            }
            None => self.build_sema_expr(ropd, ExpectAnyDerived),
        };
        (
            lopd_sema_expr_idx,
            ropd_sema_expr_idx,
            Ok(BinaryOprDynamicDispatch::builtin()),
            Ok(self.term_menu.bool_ty_ontology().into()),
        )
    }
}
