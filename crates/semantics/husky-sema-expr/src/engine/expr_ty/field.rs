use super::*;
use husky_regional_token::IdentRegionalToken;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_field_expr_ty(
        &mut self,
        owner: SynExprIdx,
        ident_token: IdentRegionalToken,
    ) -> (
        SemaExprDataResult<SemaExprData>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        let Some(owner_ty) = self.build_new_expr_ty(owner, ExpectAnyOriginal) else {
            return Err(DerivedSemaExprTypeError::FieldOwnerTypeNotInferred.into());
        };
        let field_dispatch = owner_ty
            .field_dispatch(self, ident_token.ident(), /* ad hoc: traits */ &[])
            .into_result_or(OriginalSemaExprTypeError::NoSuchField {
                owner_ty,
                ident_token,
            })?;
        let expr_ty = field_dispatch.signature().return_ty();
        Ok((field_dispatch.into(), Ok(expr_ty)))
    }
}
