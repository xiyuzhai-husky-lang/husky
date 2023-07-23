use husky_opr::SuffixOpr;

use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_explicit_application_expr_term(
        &mut self,
        expr_idx: SynExprIdx,
        function: SynExprIdx,
        argument: SynExprIdx,
    ) -> ExprTermResult<FluffyTerm> {
        // todo: implicit arguments
        let function = self
            .infer_new_expr_term(function)
            .ok_or(DerivedExprTermError::ExplicitApplicationFunctionTermNotInferred)?;
        let argument = self
            .infer_new_expr_term(argument)
            .ok_or(DerivedExprTermError::ExplicitApplicationArgumentTermNotInferred)?;
        FluffyTerm::new_application(self, expr_idx, function, argument)
            .map_err(|e| DerivedExprTermError::ExplicitApplicationTerm(e).into())
    }
}
