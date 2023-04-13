use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_binary_closed_expr_ty(
        &mut self,
        lopd: ExprIdx,
        ropd: ExprIdx,
        opr: BinaryClosedOpr,
        menu: &TermMenu,
    ) -> Result<FluffyTerm, ExprTypeError> {
        let Some(lopd_ty) = self.infer_new_expr_ty(
            lopd,
            ExpectAnyOriginal,
        ) else {
            self.infer_new_expr_ty_discarded(ropd, ExpectAnyDerived);
            Err(DerivedExprTypeError::BinaryOperationLeftOperandTypeNotInferred)?
        };
        self.infer_new_expr_ty_discarded(
            ropd,
            ExpectImplicitlyConvertible::new_pure(self, lopd_ty),
        );
        Ok(lopd_ty)
    }
}
