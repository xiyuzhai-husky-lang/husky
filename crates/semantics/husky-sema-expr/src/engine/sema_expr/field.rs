use super::*;
use husky_regional_token::IdentRegionalToken;

impl<'a> SemaExprEngine<'a> {
    pub(super) fn calc_field_expr_ty(
        &mut self,
        owner: SynExprIdx,
        dot_regional_token_idx: RegionalTokenIdx,
        ident_token: IdentRegionalToken,
    ) -> (
        SemaExprDataResult<SemaExprData>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        let (owner_sema_expr_idx, owner_ty) =
            self.build_sema_expr_with_ty(owner, ExpectAnyOriginal);
        let Some(owner_ty) = owner_ty else {
            return (
                Err(DerivedSemaExprDataError::FieldOwnerTypeNotInferred {
                    owner_sema_expr_idx,
                }
                .into()),
                Err(DerivedSemaExprTypeError::FieldOwnerTypeNotInferred.into()),
            );
        };
        let field_dispatch = owner_ty
            .field_dispatch(self, ident_token.ident(), /* ad hoc: traits */ &[])
            .into_result_or(OriginalSemaExprDataError::NoSuchField {
                owner_ty,
                ident_token,
            });
        let field_dispatch = match field_dispatch {
            Ok(field_dispatch) => field_dispatch,
            Err(e) => return (Err(e), todo!()),
        };
        let expr_ty = field_dispatch.expr_ty();
        (
            Ok(SemaExprData::Field {
                owner_sema_expr_idx,
                dot_regional_token_idx,
                ident_token,
                dispatch: field_dispatch,
            }),
            Ok(expr_ty),
        )
    }
}
