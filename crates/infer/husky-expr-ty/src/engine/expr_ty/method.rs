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
    ) -> ExprTypeResult<(ExprDisambiguation, ExprTypeResult<LocalTerm>)> {
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
        let ty_method_card = match self_expr_ty_unravelled {
            LocalTerm::Resolved(self_expr_ty_unravelled) => {
                match self
                    .db
                    .ty_method_card(self_expr_ty_unravelled, ident_token.ident())
                {
                    Ok(ty_method_card) => ty_method_card,
                    Err(e) => return Err(DerivedExprTypeError::TypeMethodTypeError(e).into()),
                }
            }
            LocalTerm::Unresolved(_) => todo!(),
        };
        if let Some(ty_method_card) = ty_method_card {
            return Ok((
                ExprDisambiguation::Method(ty_method_card.into()),
                self.calc_ty_method_expr_ty(
                    expr_idx,
                    ty_method_card,
                    self_argument,
                    implicit_arguments,
                    nonself_arguments,
                    local_term_region,
                ),
            ));
        }
        Err(OriginalExprTypeError::NoMethodForType {
            self_expr_ty_unravelled,
            ident_token,
        }
        .into())
    }

    fn calc_ty_method_expr_ty(
        &mut self,
        expr_idx: ExprIdx,
        ty_method_card: TypeMethodCard,
        self_argument: ExprIdx,
        implicit_arguments: Option<&ImplicitArgumentList>,
        nonself_arguments: ExprIdxRange,
        local_term_region: &mut LocalTermRegion,
    ) -> ExprTypeResult<LocalTerm> {
        // self.calc_ritchie_call_arguments_expr_ty(
        //     expr_idx,
        //     method_ty,
        //     todo!(),
        //     nonself_arguments,
        //     local_term_region,
        // );
        Ok(todo!())
    }
}
