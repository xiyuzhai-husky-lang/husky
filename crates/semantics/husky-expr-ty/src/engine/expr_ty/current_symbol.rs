use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn get_current_symbol_ty(
        &mut self,
        expr_idx: ExprIdx,
        current_symbol_idx: CurrentSymbolIdx,
    ) -> ExprTypeResult<LocalTerm> {
        self.current_symbol_qualified_tys
            .get(current_symbol_idx)
            .copied()
            .ok_or(DerivedExprTypeError::CurrentSymbolTypeError.into())
    }
}
