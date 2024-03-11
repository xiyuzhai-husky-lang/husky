use super::*;

impl<'a> SemaExprBuilder<'a> {
    pub(super) fn calc_explicit_application_expr_term(
        &mut self,
        function: SemaExprIdx,
        argument: SemaExprIdx,
    ) -> SemaExprTermResult<FlyTerm> {
        // todo: implicit arguments
        let function = self
            .infer_expr_term(function)
            .ok_or(DerivedSemaExprTermError::ExplicitApplicationFunctionTermNotInferred)?;
        let argument = self
            .infer_expr_term(argument)
            .ok_or(DerivedSemaExprTermError::ExplicitApplicationArgumentTermNotInferred)?;
        FlyTerm::new_application(self, function, argument)
            .map_err(|e| DerivedSemaExprTermError::ExplicitApplicationTerm(e).into())
    }
}
