use super::*;

impl<'a> SemaExprBuilder<'a> {
    pub(super) fn build_function_application_or_call_sema_expr(
        &mut self,
        syn_expr_idx: SynExprIdx,
        function_syn_expr_idx: SynExprIdx,
        expr_ty_expectation: &impl ExpectFlyTerm,
        template_arguments: Option<&SynTemplateArguments>,
        lpar_regional_token_idx: RegionalTokenIdx,
        items: &[SynCommaListItem],
        rpar_regional_token_idx: RegionalTokenIdx,
    ) -> (
        SemaExprDataResult<SemaExprData>,
        SemaExprTypeResult<FlyTerm>,
    ) {
        let (function_sema_expr_idx, outcome) = self.build_sema_expr_with_outcome(
            function_syn_expr_idx,
            ExpectEqsFunctionType::new(expr_ty_expectation.final_destination(self)),
        );
        let Some(function_expectation_outcome) = outcome else {
            for item in items {
                self.build_sema_expr(item.syn_expr_idx(), ExpectAnyDerived);
            }
            return (
                Err(
                    DerivedSemaExprDataError::ApplicationOrRitchieCallFunctionTypeNotInferred {
                        function_sema_expr_idx,
                    }
                    .into(),
                ),
                Err(
                    DerivedSemaExprTypeError::ApplicationOrRitchieCallFunctionTypeNotInferred
                        .into(),
                ),
            );
        };
        match *function_expectation_outcome.variant() {
            ExpectEqsFunctionTypeOutcomeData::TypeRitchie {
                ritchie_ty_kind,
                ref parameter_contracted_tys,
            } => {
                let ritchie_parameter_argument_matches = match self.calc_ritchie_arguments_ty(
                    syn_expr_idx,
                    parameter_contracted_tys,
                    items.iter().copied().map(Into::into),
                ) {
                    Ok(ritchie_parameter_argument_matches) => ritchie_parameter_argument_matches,
                    Err(_) => return todo!(),
                };
                (
                    Ok(SemaExprData::FunctionRitchieCall {
                        function_sema_expr_idx,
                        ritchie_ty_kind,
                        template_arguments: template_arguments.map(|_| todo!()),
                        lpar_regional_token_idx,
                        ritchie_parameter_argument_matches,
                        rpar_regional_token_idx,
                    }),
                    Ok(function_expectation_outcome.return_ty()),
                )
            }
            ExpectEqsFunctionTypeOutcomeData::ExplicitCurry {
                variance,
                parameter_hvar,
                parameter_ty,
                return_ty,
            } => {
                let argument_sema_expr_idx = match items.len() {
                    0 => self.build_unit_sema_expr(
                        syn_expr_idx,
                        lpar_regional_token_idx,
                        rpar_regional_token_idx,
                        ExpectSubtypeOrEqual::new(parameter_ty),
                    ),
                    1 => self.build_sema_expr(
                        items.first().expect("len is 1").syn_expr_idx(),
                        ExpectCoersion::new_const(parameter_ty),
                    ),
                    // parameter_ty must be a tuple
                    // distribute the types for a tuple
                    _ => todo!(),
                };
                (
                    Ok(SemaExprData::FunctionApplication {
                        function_sema_expr_idx,
                        argument_sema_expr_idx,
                    }),
                    Ok(return_ty),
                )
            }
        }
    }

    pub(super) fn build_function_call_sema_expr(
        &mut self,
        syn_expr_idx: SynExprIdx,
        function_syn_expr_idx: SynExprIdx,
        final_destination: FinalDestination,
        template_arguments: Option<&SynTemplateArguments>,
        lpar_regional_token_idx: RegionalTokenIdx,
        items: &[SynCallListItem],
        rpar_regional_token_idx: RegionalTokenIdx,
    ) -> (
        SemaExprDataResult<SemaExprData>,
        SemaExprTypeResult<FlyTerm>,
    ) {
        let (function_sema_expr_idx, outcome) = self.build_sema_expr_with_outcome(
            function_syn_expr_idx,
            ExpectEqsRitchieType::new(final_destination),
        );
        let Some(function_expectation_outcome) = outcome else {
            for item in items {
                self.build_sema_expr_with_ty(item.argument_expr_idx(), ExpectAnyDerived);
            }
            return (
                Err(todo!()),
                Err(
                    DerivedSemaExprTypeError::ApplicationOrRitchieCallFunctionTypeNotInferred
                        .into(),
                ),
            );
        };
        let ritchie_parameter_argument_matches = match self.calc_ritchie_arguments_ty(
            syn_expr_idx,
            function_expectation_outcome.parameter_contracted_tys(),
            items.iter().copied(),
        ) {
            Ok(ritchie_parameter_argument_matches) => ritchie_parameter_argument_matches,
            Err(e) => return todo!(),
        };
        let ritchie_ty_kind = function_expectation_outcome.ritchie_ty_kind();
        let return_ty = function_expectation_outcome.return_ty();
        let data = SemaExprData::FunctionRitchieCall {
            function_sema_expr_idx,
            ritchie_ty_kind,
            template_arguments: template_arguments.map(|_| todo!()),
            lpar_regional_token_idx,
            ritchie_parameter_argument_matches,
            rpar_regional_token_idx,
        };
        (Ok(data), Ok(return_ty))
    }
}
