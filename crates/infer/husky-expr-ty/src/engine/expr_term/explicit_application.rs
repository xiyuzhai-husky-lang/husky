use husky_opn_syntax::SuffixOpr;

use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_explicit_application_expr_term(
        &mut self,
        function: ExprIdx,
        argument: ExprIdx,
    ) -> ExprTermResult<LocalTerm> {
        let function_term = self
            .infer_new_expr_term(function)
            .ok_or(DerivedExprTermError::Todo)?;
        let argument_term = self
            .infer_new_expr_term(argument)
            .ok_or(DerivedExprTermError::Todo)?;
        Ok(todo!())
    }
}
