use super::*;
use husky_token::IdentToken;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_method_expr_ty(
        &mut self,
        expr_idx: ExprIdx,
        self_argument: ExprIdx,
        ident_token: IdentToken,
        implicit_arguments: Option<&ImplicitArgumentList>,
        nonself_arguments: ExprIdxRange,
        local_term_region: &mut LocalTermRegion,
    ) -> ExprTypeResult<LocalTerm> {
        let Some(self_expr_ty) =
            self.infer_new_expr_ty( self_argument, ExpectAnyOriginal,local_term_region)
            else {
                if let Some(implicit_arguments) = implicit_arguments {
                    todo!()
                }
                for argument in nonself_arguments {
                    self.infer_new_expr_ty_discarded(argument, ExpectAnyDerived, local_term_region);
                }
                return Err(DerivedExprTypeError::MethodOwnerTypeNotInferred.into())
            };
        let self_expr_ty_unravelled =
            self_expr_ty.unravel_borrow(self.db, local_term_region.unresolved_terms());
        let method_ty = match self_expr_ty_unravelled {
            LocalTerm::Resolved(self_expr_ty_unravelled) => {
                match self
                    .db
                    .ty_method_card(self_expr_ty_unravelled, ident_token.ident())
                {
                    Ok(_) => todo!(),
                    Err(e) => return Err(DerivedExprTypeError::TypeMethodTypeError(e).into()),
                }
            }
            LocalTerm::Unresolved(_) => todo!(),
        };
        self.calc_ritchie_call_arguments_expr_ty(
            expr_idx,
            method_ty,
            todo!(),
            nonself_arguments,
            local_term_region,
        );
        Ok(todo!())
    }
}
