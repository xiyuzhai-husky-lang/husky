use super::*;
use husky_token::IdentToken;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_associated_item_ty(
        &mut self,
        expr_idx: ExprIdx,
        parent_expr_idx: ExprIdx,
        ident_token: IdentToken,
    ) -> ExprTypeResult<(ExprDisambiguation, ExprTypeResult<FluffyTerm>)> {
        self.infer_new_expr_ty_discarded(parent_expr_idx, ExpectEqsCategory::new_any_sort());
        let parent_term = self
            .infer_new_expr_term(parent_expr_idx)
            .ok_or(DerivedExprTypeError::UnableToInferAssociatedItemParentTerm)?;
        let associated_item_disambiguation = parent_term.disambiguate_associated_item(
            self,
            ident_token.ident(),
            /*ad hoc */ &[],
        );
        todo!()
    }
}
