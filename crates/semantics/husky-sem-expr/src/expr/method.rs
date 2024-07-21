use super::*;
use dispatch::method::{HasFlyMethodDispatch, MethodFlySignature};
use husky_regional_token::IdentRegionalToken;

impl<'a> SemExprBuilder<'a> {
    pub(super) fn calc_method_application_or_call_ty(
        &mut self,
        expr_idx: SynExprIdx,
        self_argument: SynExprIdx,
        dot_regional_token_idx: RegionalTokenIdx,
        ident_token: IdentRegionalToken,
        template_arguments: Option<&SynTemplateArguments>,
        lpar_regional_token_idx: RegionalTokenIdx,
        list_items: &[SynCommaListItem],
        rpar_regional_token_idx: RegionalTokenIdx,
    ) -> (SemExprDataResult<SemExprData>, SemExprTypeResult<FlyTerm>) {
        let (self_argument, self_argument_ty, outcome) =
            self.build_expr_with_ty_and_outcome(self_argument, ExpectAnyOriginal);
        let Some(self_expr_ty) = self_argument_ty else {
            if let Some(template_arguments) = template_arguments {
                todo!()
            }
            let list_items = list_items
                .iter()
                .map(|list_item| self.build_expr(list_item.syn_expr_idx(), ExpectAnyDerived))
                .collect();
            return (
                Err(DerivedSemExprDataError::MethodOwnerTypeNotInferred {
                    self_argument,
                    list_items,
                }
                .into()),
                Err(DerivedSemExprTypeError::MethodOwnerTypeNotInferred.into()),
            );
        };
        let instance_dispatch = match self_expr_ty
            .method_dispatch(self, expr_idx, ident_token)
            .into_result_or(OriginalSemExprDataError::NoSuchMethod {
                self_expr_ty,
                ident_token,
            }) {
            Ok(method_dynamic_dispatch) => method_dynamic_dispatch,
            Err(e) => return (Err(e), Err(todo!())),
        };
        match instance_dispatch.signature() {
            MethodFlySignature::TypeMethodRitchie(signature) => {
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
                    Ok(SemExprData::MethodRitchieCall {
                        self_argument,
                        self_contract: signature.self_value_parameter.contract,
                        dot_regional_token_idx,
                        ident_token,
                        dispatch: instance_dispatch,
                        template_arguments: template_arguments.map(|_| todo!()),
                        lpar_regional_token_idx,
                        ritchie_parameter_argument_matches,
                        rpar_regional_token_idx,
                    }),
                    Ok(return_ty),
                )
            }
            MethodFlySignature::TraitForTypeMethodRitchie(signature) => {
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
                    Ok(SemExprData::MethodRitchieCall {
                        self_argument,
                        self_contract: signature.self_value_parameter.contract,
                        dot_regional_token_idx,
                        ident_token,
                        dispatch: instance_dispatch,
                        template_arguments: template_arguments.map(|_| todo!()),
                        lpar_regional_token_idx,
                        ritchie_parameter_argument_matches,
                        rpar_regional_token_idx,
                    }),
                    Ok(return_ty),
                )
            }
        }
    }
}
