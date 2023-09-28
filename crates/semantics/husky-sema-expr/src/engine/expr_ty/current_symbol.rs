use super::*;

impl<'a> SemaExprEngine<'a> {
    pub(super) fn get_current_symbol_ty(
        &mut self,
        expr_idx: SynExprIdx,
        current_symbol_idx: SynCurrentSymbolIdx,
    ) -> SemaExprTypeResult<FluffyTerm> {
        Ok(self
            .symbol_tys
            .current_symbol_map()
            .get(current_symbol_idx)
            .copied()
            .ok_or(DerivedSemaExprTypeError::CurrentSymbolTypeError)?
            .into())
    }
}
