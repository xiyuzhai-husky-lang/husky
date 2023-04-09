use super::*;
use husky_token::IdentToken;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_field_expr_ty(
        &mut self,
        owner: ExprIdx,
        ident_token: IdentToken,
    ) -> ExprTypeResult<FluffyTerm> {
        let Some(owner_ty) = self.infer_new_expr_ty(owner, ExpectAnyOriginal,  )else {
            return Err(DerivedExprTypeError::FieldOwnerTypeNotInferred.into())
        };
        todo!()
        // let owner_ty_unravelled =
        //     owner_ty.unravel_borrow(self.db, self.fluffy_term_region.porous_terms());
        // match owner_ty_unravelled {
        //     FluffyTerm::Term(owner_ty_unravelled) => {
        //         match self.db.field_ty(owner_ty_unravelled, ident_token.ident()) {
        //             Ok(Some(field_ty)) => Ok(field_ty.into()),
        //             Ok(None) => Err(OriginalExprTypeError::NoSuchField.into()),
        //             Err(e) => Err(DerivedExprTypeError::FieldTypeTermError(e).into()),
        //         }
        //     }
        //     FluffyTerm::Unresolved(_) => todo!(),
        //     _ => todo!(),
        // }
    }
}
