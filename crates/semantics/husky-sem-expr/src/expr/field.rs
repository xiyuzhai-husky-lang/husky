use super::*;
use husky_regional_token::IdentRegionalToken;

impl<'a> SemExprBuilder<'a> {
    pub(super) fn build_field_expr(
        &mut self,
        owner: SynExprIdx,
        dot_regional_token_idx: RegionalTokenIdx,
        ident_token: IdentRegionalToken,
    ) -> (SemExprDataResult<SemExprData>, SemExprTypeResult<FlyTerm>) {
        let (self_argument, self_argument_ty) = self.build_expr_with_ty(owner, ExpectAnyOriginal);
        let Some(self_argument_ty) = self_argument_ty else {
            return (
                Err(DerivedSemExprDataError::FieldOwnerTypeNotInferred { self_argument }.into()),
                Err(DerivedSemExprTypeError::FieldOwnerTypeNotInferred.into()),
            );
        };
        let dispatch = self_argument_ty
            .field_dispatch(self, ident_token.ident(), /* ad hoc: traits */ &[])
            .into_result_or(OriginalSemExprDataError::NoSuchField {
                self_argument_ty,
                ident_token,
            });
        let dispatch = match dispatch {
            Ok(field_dispatch) => field_dispatch,
            Err(e) => return (Err(e), todo!()),
        };
        let expr_ty_result = dispatch.expr_ty_result();
        (
            Ok(SemExprData::Field {
                self_argument,
                self_argument_ty,
                dot_regional_token_idx,
                ident_token,
                dispatch,
            }),
            expr_ty_result.map_err(Into::into),
        )
    }
}
