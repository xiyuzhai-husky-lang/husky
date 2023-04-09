use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn get_current_symbol_ty(
        &mut self,
        expr_idx: ExprIdx,
        current_symbol_idx: CurrentSymbolIdx,
    ) -> ExprTypeResult<FluffyTerm> {
        Ok(self
            .current_symbol_place_tys
            .get(current_symbol_idx)
            .copied()
            .ok_or(DerivedExprTypeError::CurrentSymbolTypeError)?
            .into())
    }
}
