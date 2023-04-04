use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_binary_comparison_expr_ty(
        &mut self,
        lopd: ExprIdx,
        ropd: ExprIdx,
        local_term_region: &mut LocalTermRegion,
    ) -> Result<LocalTerm, ExprTypeError> {
        let lopd_ty = self.infer_new_expr_ty(lopd, ExpectAnyOriginal, local_term_region);
        match lopd_ty {
            Some(destination) => self.infer_new_expr_ty_discarded(
                ropd,
                ExpectImplicitlyConvertible::new_ad_hoc(destination),
                local_term_region,
            ),
            None => self.infer_new_expr_ty_discarded(ropd, ExpectAnyDerived, local_term_region),
        };
        Ok(self.term_menu.bool_ty_ontology().into())
    }
}
