use super::*;
use husky_regional_token::IdentRegionalToken;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_method_application_or_call_ty(
        &mut self,
        expr_idx: SynExprIdx,
        self_argument: SynExprIdx,
        ident_token: IdentRegionalToken,
        template_arguments: Option<&SynTemplateArgumentList>,
        list_items: &[SynCommaListItem],
    ) -> (
        SemaExprDataResult<SemaExprData>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        let (self_argument_sema_expr_idx, self_argument_ty) =
            self.build_new_expr_ty(self_argument, ExpectAnyOriginal);
        let Some(self_expr_ty) = self_argument_ty else {
            if let Some(generic_arguments) = template_arguments {
                todo!()
            }
            let list_items = list_items
                .iter()
                .map(|list_item| {
                    self.build_new_expr_ty_discarded(list_item.expr_idx(), ExpectAnyDerived)
                })
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
        let method_dispatch = self_expr_ty
            .method_dispatch(self, expr_idx, ident_token)
            .into_result_or(OriginalSemaExprTypeError::NoMethodForType {
                self_expr_ty,
                ident_token,
            })?;
        match method_dispatch.signature() {
            MethodFluffySignature::MethodFn(signature) => {
                let ritchie_parameter_argument_matches = self.calc_ritchie_arguments_ty(
                    expr_idx,
                    signature.nonself_parameter_contracted_tys(),
                    list_items.iter().copied().map(Into::into),
                );
                let return_ty = signature.return_ty();
                (
                    Ok(SemaExprData::MethodFnCall {
                        method_dispatch,
                        ritchie_parameter_argument_matches,
                    }),
                    Ok(return_ty),
                )
            }
            MethodFluffySignature::MethodFunction(signature) => todo!(),
            MethodFluffySignature::MethodGn => todo!(),
        }
    }
}
