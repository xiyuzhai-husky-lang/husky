use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_binary_comparison_expr_ty(
        &mut self,
        lopd: SynExprIdx,
        ropd: SynExprIdx,
    ) -> SemaExprResult<FluffyTerm> {
        let lopd_ty = self.infer_new_expr_ty(lopd, ExpectAnyOriginal);
        match lopd_ty {
            Some(destination) => {
                self.build_new_expr_ty_discarded(ropd, ExpectCoersion::new_pure(self, destination))
            }
            None => self.build_new_expr_ty_discarded(ropd, ExpectAnyDerived),
        };
        Ok(self.term_menu.bool_ty_ontology().into())
    }
}
