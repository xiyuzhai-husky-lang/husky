use super::*;

impl<'a> SemExprBuilder<'a> {
    pub(super) fn get_current_variable_ty(
        &mut self,
        expr_idx: SynExprIdx,
        current_variable_idx: CurrentVariableIdx,
    ) -> SemExprTypeResult<FlyTerm> {
        Ok(self
            .symbol_tys()
            .current_variable_map()
            .get(current_variable_idx)
            .copied()
            .ok_or(DerivedSemExprTypeError::CurrentSynSymbolTypeError)?
            .into())
    }
}
