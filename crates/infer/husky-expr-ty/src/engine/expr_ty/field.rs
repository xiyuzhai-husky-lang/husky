use super::*;
use husky_token::IdentToken;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_field_expr_ty(
        &mut self,
        owner: ExprIdx,
        ident_token: IdentToken,
        local_term_region: &mut LocalTermRegion,
    ) -> ExprTypeResult<LocalTerm> {
        let Some(owner_ty) = self.infer_new_expr_ty(owner, ExpectAnyOriginal, local_term_region)else {
            return Err(DerivedExprTypeError::FieldOwnerTypeNotInferred.into())
        };
        let owner_ty_unravelled =
            owner_ty.unravel_borrow(self.db, local_term_region.unresolved_terms());
        match owner_ty_unravelled {
            LocalTerm::Resolved(owner_ty_unravelled) => {
                match self.db.field_ty(owner_ty_unravelled, ident_token.ident()) {
                    Ok(Some(_)) => todo!(),
                    Ok(None) => todo!(),
                    Err(e) => Err(DerivedExprTypeError::FieldTypeTermError(e).into()),
                }
            }
            LocalTerm::Unresolved(_) => todo!(),
        }
    }
}
