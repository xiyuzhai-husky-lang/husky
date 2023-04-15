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
        let (disambiguation, ty_result) =
            owner_ty.field_ty(self, ident_token.ident(), /* ad hoc: traits */ &[])?;
        Ok((disambiguation.into(), ty_result.map_err(Into::into)))
    }
}
