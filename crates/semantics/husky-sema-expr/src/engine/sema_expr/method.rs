use super::*;
use husky_regional_token::IdentRegionalToken;

impl<'a> SemaExprEngine<'a> {
    pub(super) fn calc_method_application_or_call_ty(
        &mut self,
        expr_idx: SynExprIdx,
        self_argument: SynExprIdx,
        dot_regional_token_idx: RegionalTokenIdx,
        ident_token: IdentRegionalToken,
        template_arguments: Option<&SynTemplateArgumentList>,
        lpar_regional_token_idx: RegionalTokenIdx,
        list_items: &[SynCommaListItem],
        rpar_regional_token_idx: RegionalTokenIdx,
    ) -> (
        SemaExprDataResult<SemaExprData>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        let (self_argument_sema_expr_idx, self_argument_ty) =
            self.build_sema_expr_with_ty(self_argument, ExpectAnyOriginal);
        let Some(self_expr_ty) = self_argument_ty else {
            if let Some(generic_arguments) = template_arguments {
                todo!()
            }
            let list_items = list_items
                .iter()
                .map(|list_item| self.build_sema_expr(list_item.syn_expr_idx(), ExpectAnyDerived))
                .collect();
            return (
                Err(DerivedSemaExprDataError::MethodOwnerTypeNotInferred {
                    self_argument_sema_expr_idx,
                    list_item_sema_expr_idxs: list_items,
                }
                .into()),
                Err(DerivedSemaExprTypeError::MethodOwnerTypeNotInferred.into()),
            );
        };
        let method_dynamic_dispatch = match self_expr_ty
            .method_dispatch(self, expr_idx, ident_token)
            .into_result_or(OriginalSemaExprDataError::NoSuchMethod {
                self_expr_ty,
                ident_token,
            }) {
            Ok(method_dynamic_dispatch) => method_dynamic_dispatch,
            Err(e) => return (Err(e), Err(todo!())),
        };
        match method_dynamic_dispatch.signature() {
            MethodFluffySignature::MethodFn(signature) => {
                let return_ty = signature.return_ty();
                let ritchie_parameter_argument_matches = match self.calc_ritchie_arguments_ty(
                    expr_idx,
                    signature.nonself_parameter_contracted_tys(),
                    list_items.iter().copied().map(Into::into),
                ) {
                    Ok(ritchie_parameter_argument_matches) => ritchie_parameter_argument_matches,
                    Err(_) => todo!(),
                };
                (
                    Ok(SemaExprData::MethodFnCall {
                        self_argument_sema_expr_idx,
                        dot_regional_token_idx,
                        ident_token,
                        dispatch: method_dynamic_dispatch,
                        template_arguments: template_arguments.map(|_| todo!()),
                        lpar_regional_token_idx,
                        ritchie_parameter_argument_matches,
                        rpar_regional_token_idx,
                    }),
                    Ok(return_ty),
                )
            }
            MethodFluffySignature::MethodGn => todo!(),
        }
    }
}
