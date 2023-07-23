use super::*;
use husky_token::IdentToken;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_method_application_or_call_ty(
        &mut self,
        expr_idx: SynExprIdx,
        self_argument: SynExprIdx,
        ident_token: IdentToken,
        generic_arguments: Option<&SynGenericArgumentList>,
        explicit_arguments: &[SynCommaListItem],
    ) -> ExprTypeResult<(ExprDisambiguation, ExprTypeResult<FluffyTerm>)> {
        let Some(self_expr_ty) =
            self.infer_new_expr_ty( self_argument, ExpectAnyOriginal)
            else {
                if let Some(generic_arguments) = generic_arguments {
                    todo!()
                }
                for argument in explicit_arguments {
                    self.infer_new_expr_ty_discarded(argument.expr_idx(), ExpectAnyDerived);
                }
                return Err(DerivedExprTypeError::MethodOwnerTypeNotInferred.into())
            };
        let method_dispatch = self_expr_ty
            .method_dispatch(self, expr_idx, ident_token)
            .into_result_or(OriginalExprTypeError::NoMethodForType {
                self_expr_ty,
                ident_token,
            })?;
        let return_ty = match method_dispatch.signature() {
            MethodFluffySignature::MethodFn(signature) => {
                self.calc_ritchie_arguments_expr_ty(
                    expr_idx,
                    signature.nonself_parameter_contracted_tys(),
                    explicit_arguments.iter().copied().map(Into::into),
                );
                signature.return_ty()
            }
            MethodFluffySignature::MethodFunction(signature) => todo!(),
        };
        Ok((method_dispatch.into(), Ok(return_ty)))
    }
}
