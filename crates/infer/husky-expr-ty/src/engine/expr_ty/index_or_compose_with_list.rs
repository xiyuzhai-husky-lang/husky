use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_index_or_compose_with_list_expr_ty(
        &mut self,
        expr_idx: ExprIdx,
        owner: ExprIdx,
        indices: ExprIdxRange,
        local_term_region: &mut LocalTermRegion,
    ) -> Result<LocalTerm, ExprTypeError> {
        let Some(owner_ty) = self.infer_new_expr_ty(owner, ExpectAnyOriginal, local_term_region)
        else  {
            for index in indices {
                self.infer_new_expr_ty(index, ExpectAnyDerived, local_term_region);
            }
            return Err(
                DerivedExprTypeError::ApplicationOrRitchieCallFunctionTypeNotInferred
                    .into(),
            )
        };
        todo!()
    }
}
