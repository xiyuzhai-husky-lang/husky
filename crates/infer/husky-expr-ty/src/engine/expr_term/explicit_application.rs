use husky_opn_syntax::SuffixOpr;

use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_explicit_application_expr_term(
        &mut self,
        function: ExprIdx,
        argument: ExprIdx,
    ) -> ExprTermResult<LocalTerm> {
        // todo: implicit arguments
        let function = self
            .infer_new_expr_term(function)
            .ok_or(DerivedExprTermError::Todo)?;
        let argument = self
            .infer_new_expr_term(argument)
            .ok_or(DerivedExprTermError::Todo)?;
        LocalTerm::new_application(self.db, function, argument)
            .map_err(|e| DerivedExprTermError::ExplicitApplicationTerm(e).into())
    }
}
