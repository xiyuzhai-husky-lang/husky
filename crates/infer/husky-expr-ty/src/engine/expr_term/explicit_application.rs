use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_explicit_application_expr_term(
        &mut self,
        function: ExprIdx,
        argument: ExprIdx,
    ) -> ExprTermResult<LocalTerm> {
        let function_term: LocalTerm = match self.expr_region_data[function] {
            Expr::IndexOrComposeWithList {
                owner,
                lbox_token_idx,
                items,
                rbox_token_idx,
            } => todo!(),
            Expr::BoxList {
                lbox_token_idx,
                items,
                rbox_token_idx,
            } if items.len() == 0 => {
                todo!()
            }
            Expr::BoxColonList {
                lbox_token_idx,
                colon_token_idx,
                items,
                rbox_token_idx,
            } => todo!(),
            _ => self
                .infer_new_expr_term(function)
                .ok_or(DerivedExprTermError::Todo)?,
        };
        let Some(argument_term) = self.infer_new_expr_term(argument)
                else {
                    todo!()
                };
        Ok(todo!())
    }
}
