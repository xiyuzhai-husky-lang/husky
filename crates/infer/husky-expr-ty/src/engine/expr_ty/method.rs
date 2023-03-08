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
            self.infer_new_expr_ty_with_resolved_ty_returned( self_argument, ExpectAnyOriginal,local_term_region)
            else {
                if let Some(implicit_arguments) = implicit_arguments {
                    todo!()
                }
                for argument in nonself_arguments {
                    self.infer_new_expr_ty_discarded(argument, ExpectAnyDerived, local_term_region);
                }
                return Err(DerivedExprTypeError::MethodOwnerTypeNotInferred.into())
            };
        let method_ty = todo!();
        //  match self.db.ty_method_ty(self_expr_ty, ident_token.ident()) {
        //     Ok(_) => todo!(),
        //     Err(error) => {
        //         return Err(match error {
        //             TypeError::Original(error) => {
        //                 OriginalExprTypeError::TypeMethodTypeError(error).into()
        //             }
        //             TypeError::Derived(error) => {
        //                 DerivedExprTypeError::TypeMethodTypeError(error).into()
        //             }
        //         })
        //     }
        // };
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
