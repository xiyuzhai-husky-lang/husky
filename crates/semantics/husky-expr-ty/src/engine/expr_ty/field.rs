use super::*;
use husky_token::IdentToken;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_field_expr_ty(
        &mut self,
        owner: ExprIdx,
        ident_token: IdentToken,
    ) -> ExprTypeResult<(ExprDisambiguation, ExprTypeResult<FluffyTerm>)> {
        let Some(owner_ty) = self.infer_new_expr_ty(owner, ExpectAnyOriginal,  )else {
            return Err(DerivedExprTypeError::FieldOwnerTypeNotInferred.into())
        };
        let disambiguation = owner_ty
            .field_dispatch(self, ident_token.ident(), /* ad hoc: traits */ &[])
            .into_result_or(OriginalExprTypeError::NoSuchField {
                owner_ty,
                ident_token,
            })?;
        let expr_ty = disambiguation.signature().ty();
        Ok((disambiguation.into(), Ok(expr_ty)))
    }
}
