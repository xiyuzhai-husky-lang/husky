use super::*;
use husky_regional_token::IdentRegionalToken;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_field_expr_ty(
        &mut self,
        owner: SynExprIdx,
        ident_token: IdentRegionalToken,
    ) -> (SemaExprResult<SemaExprData>, SemaExprResult<FluffyTerm>) {
        let Some(owner_ty) = self.infer_new_expr_ty(owner, ExpectAnyOriginal) else {
            return Err(DerivedSemaExprError::FieldOwnerTypeNotInferred.into());
        };
        let field_dispatch = owner_ty
            .field_dispatch(self, ident_token.ident(), /* ad hoc: traits */ &[])
            .into_result_or(OriginalSemaExprError::NoSuchField {
                owner_ty,
                ident_token,
            })?;
        let expr_ty = field_dispatch.signature().return_ty();
        Ok((field_dispatch.into(), Ok(expr_ty)))
    }
}
