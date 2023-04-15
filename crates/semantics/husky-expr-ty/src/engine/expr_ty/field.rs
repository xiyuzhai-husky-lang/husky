use super::*;
use husky_token::IdentToken;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_field_or_memo_expr_ty(
        &mut self,
        owner: ExprIdx,
        ident_token: IdentToken,
    ) -> ExprTypeResult<(ExprDisambiguation, ExprTypeResult<FluffyTerm>)> {
        let Some(owner_ty) = self.infer_new_expr_ty(owner, ExpectAnyOriginal,  )else {
            return Err(DerivedExprTypeError::FieldOwnerTypeNotInferred.into())
        };
        if let Some(field_ty) = owner_ty.field_ty(self, ident_token.ident())? {
            return Ok((
                ExprDisambiguation::Field(FluffyFieldDisambiguation::Field),
                Ok(field_ty),
            ));
        }
        Err(OriginalExprTypeError::TodoMemo)?
    }
}
