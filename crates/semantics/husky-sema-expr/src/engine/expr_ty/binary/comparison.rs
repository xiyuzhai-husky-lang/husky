use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_binary_comparison_expr_ty(
        &mut self,
        lopd: SynExprIdx,
        ropd: SynExprIdx,
    ) -> (SemaExprIdx, SemaExprIdx, SemaExprTypeResult<FluffyTerm>) {
        let (lopd_sema_expr_idx, lopd_ty) = self.build_new_expr_ty(lopd, ExpectAnyOriginal);
        let ropd_sema_expr_idx = match lopd_ty {
            Some(destination) => {
                self.build_new_expr_ty_discarded(ropd, ExpectCoersion::new_pure(self, destination))
            }
            None => self.build_new_expr_ty_discarded(ropd, ExpectAnyDerived),
        };
        (
            lopd_sema_expr_idx,
            ropd_sema_expr_idx,
            Ok(self.term_menu.bool_ty_ontology().into()),
        )
    }
}
