use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_binary_comparison_expr_ty(
        &mut self,
        lopd: ExprIdx,
        ropd: ExprIdx,
    ) -> Result<FluffyTerm, ExprTypeError> {
        let lopd_ty = self.infer_new_expr_ty(lopd, ExpectAnyOriginal);
        match lopd_ty {
            Some(destination) => self.infer_new_expr_ty_discarded(
                ropd,
                ExpectImplicitlyConvertible::new_pure(self, destination),
            ),
            None => self.infer_new_expr_ty_discarded(ropd, ExpectAnyDerived),
        };
        Ok(self.term_menu.bool_ty_ontology().into())
    }
}
