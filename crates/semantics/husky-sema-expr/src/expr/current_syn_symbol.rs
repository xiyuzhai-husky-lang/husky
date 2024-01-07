use super::*;

impl<'a> SemaExprEngine<'a> {
    pub(super) fn get_current_syn_symbol_ty(
        &mut self,
        expr_idx: SynExprIdx,
        current_syn_symbol_idx: CurrentSynSymbolIdx,
    ) -> SemaExprTypeResult<FluffyTerm> {
        Ok(self
            .symbol_tys()
            .current_syn_symbol_map()
            .get(current_syn_symbol_idx)
            .copied()
            .ok_or(DerivedSemaExprTypeError::CurrentSynSymbolTypeError)?
            .into())
    }
}
