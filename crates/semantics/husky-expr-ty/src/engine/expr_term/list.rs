use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_list_expr_term(
        &mut self,
        expr_idx: ExprIdx,
        items: ExprIdxRange,
    ) -> ExprTermResult<FluffyTerm> {
        match self
            .expr_disambiguation(expr_idx)
            .map_err(|_| DerivedExprTermError::Todo)?
            .list_expr_disambiguation()
            .expect("seriously?")
        {
            ListExprDisambiguation::NewList => todo!(),
            ListExprDisambiguation::ListFunctor => {
                assert_eq!(items.len(), 0);
                Ok(self.term_menu.list_ty_ontology().into())
            }
            ListExprDisambiguation::ArrayFunctor => todo!(),
        }
    }
}
