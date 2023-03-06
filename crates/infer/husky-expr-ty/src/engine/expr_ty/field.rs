use super::*;
use husky_token::IdentifierToken;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_field_expr_ty(
        &mut self,
        owner: ExprIdx,
        ident_token: IdentifierToken,
        local_term_region: &mut LocalTermRegion,
    ) -> ExprTypeResult<LocalTerm> {
        let Some(owner_ty) = self.infer_new_expr_ty(owner, ExpectAnyOriginal, local_term_region)else {
            return Err(DerivedExprTypeError::FieldOwnerTypeNotInferred.into())
        };
        match owner_ty {
            LocalTerm::Resolved(owner_ty) => {
                todo!()
                // let field_ty = self.db.field_ty(owner_ty, ident_token.ident());
                // match field_ty {
                //     Ok(_) => todo!(),
                //     Err(e) => Err(match e {
                //         TypeError::Original(error) => {
                //             OriginalExprTypeError::FieldTypeError(error).into()
                //         }
                //         TypeError::Derived(error) => {
                //             DerivedExprTypeError::FieldTypeError(error).into()
                //         }
                //     }),
                // }
            }
            LocalTerm::Unresolved(_) => todo!(),
        }
    }
}
