use super::*;
use husky_token::IdentToken;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_method_application_or_call_ty(
        &mut self,
        expr_idx: ExprIdx,
        self_argument: ExprIdx,
        ident_token: IdentToken,
        implicit_arguments: Option<&ImplicitArgumentList>,
        explicit_arguments: &[CommaListItem],
    ) -> ExprTypeResult<(ExprDisambiguation, ExprTypeResult<FluffyTerm>)> {
        let Some(self_expr_ty) =
            self.infer_new_expr_ty( self_argument, ExpectAnyOriginal)
            else {
                if let Some(implicit_arguments) = implicit_arguments {
                    todo!()
                }
                for argument in explicit_arguments {
                    self.infer_new_expr_ty_discarded(argument.expr_idx(), ExpectAnyDerived);
                }
                return Err(DerivedExprTypeError::MethodOwnerTypeNotInferred.into())
            };
        let method_disambiguation = self_expr_ty
            .method_disambiguation(self, ident_token.ident(), /* ad hoc */ &[])
            .into_result_or(OriginalExprTypeError::NoMethodForType {
                self_expr_ty,
                ident_token,
            })?;
        let return_ty = match method_disambiguation.signature() {
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
        Ok((method_disambiguation.into(), Ok(return_ty)))
    }
}
