use super::*;
use husky_regional_token::IdentRegionalToken;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_method_application_or_call_ty(
        &mut self,
        expr_idx: SynExprIdx,
        self_argument: SynExprIdx,
        ident_token: IdentRegionalToken,
        generic_arguments: Option<&SynTemplateArgumentList>,
        explicit_arguments: &[SynCommaListItem],
    ) -> SemaExprResult<(SynExprDisambiguation, SemaExprResult<FluffyTerm>)> {
        let Some(self_expr_ty) = self.infer_new_expr_ty(self_argument, ExpectAnyOriginal) else {
            if let Some(generic_arguments) = generic_arguments {
                todo!()
            }
            for argument in explicit_arguments {
                self.infer_new_expr_ty_discarded(argument.expr_idx(), ExpectAnyDerived);
            }
            return Err(DerivedSemaExprError::MethodOwnerTypeNotInferred.into());
        };
        let method_dispatch = self_expr_ty
            .method_dispatch(self, expr_idx, ident_token)
            .into_result_or(OriginalSemaExprError::NoMethodForType {
                self_expr_ty,
                ident_token,
            })?;
        match method_dispatch.signature() {
            MethodFluffySignature::MethodFn(signature) => {
                let ritchie_parameter_argument_matches = self.calc_ritchie_arguments_ty(
                    expr_idx,
                    signature.nonself_parameter_contracted_tys(),
                    explicit_arguments.iter().copied().map(Into::into),
                );
                let return_ty = signature.return_ty();
                Ok((
                    MethodCallOrApplicationDisambiguation::MethodCall {
                        method_dispatch,
                        ritchie_parameter_argument_matches,
                    }
                    .into(),
                    Ok(return_ty),
                ))
            }
            MethodFluffySignature::MethodFunction(signature) => todo!(),
            MethodFluffySignature::MethodGn => todo!(),
        }
    }
}
