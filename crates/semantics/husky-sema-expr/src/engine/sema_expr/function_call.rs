use super::*;

impl<'a> SemaExprEngine<'a> {
    pub(super) fn build_function_application_or_call_sema_expr(
        &mut self,
        syn_expr_idx: SynExprIdx,
        function_syn_expr_idx: SynExprIdx,
        expr_ty_expectation: &impl ExpectFluffyTerm,
        template_arguments: Option<&SynTemplateArguments>,
        lpar_regional_token_idx: RegionalTokenIdx,
        items: &[SynCommaListItem],
        rpar_regional_token_idx: RegionalTokenIdx,
    ) -> (
        SemaExprDataResult<SemaExprData>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        let (function_sema_expr_idx, outcome) = self.build_sema_expr_with_outcome(
            function_syn_expr_idx,
            ExpectEqsFunctionType::new(expr_ty_expectation.final_destination(self)),
        );
        let Some(outcome) = outcome else {
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
        match *outcome.variant() {
            ExpectEqsFunctionTypeOutcomeData::Ritchie {
                ritchie_kind,
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
                    Ok(SemaExprData::FunctionFnCall {
                        function_sema_expr_idx,
                        template_arguments: template_arguments.map(|_| todo!()),
                        lpar_regional_token_idx,
                        ritchie_parameter_argument_matches,
                        rpar_regional_token_idx,
                    }),
                    Ok(outcome.return_ty()),
                )
            }
            ExpectEqsFunctionTypeOutcomeData::Curry {
                variance,
                parameter_rune: parameter_symbol,
                parameter_ty,
                return_ty,
            } => {
                let argument_sema_expr_idx = match items.len() {
                    0 => self.new_unit_expr(
                        lpar_regional_token_idx,
                        rpar_regional_token_idx,
                        parameter_ty,
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
        expr_idx: SynExprIdx,
        function: SynExprIdx,
        final_destination: FinalDestination,
        template_arguments: Option<&SynTemplateArguments>,
        lpar_regional_token_idx: RegionalTokenIdx,
        items: &[SynCallListItem],
        rpar_regional_token_idx: RegionalTokenIdx,
    ) -> (
        SemaExprDataResult<SemaExprData>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        let (function_sema_expr_idx, outcome) = self
            .build_sema_expr_with_outcome(function, ExpectEqsRitchieType::new(final_destination));
        let Some(outcome) = outcome else {
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
            expr_idx,
            outcome.parameter_contracted_tys(),
            items.iter().copied(),
        ) {
            Ok(ritchie_parameter_argument_matches) => ritchie_parameter_argument_matches,
            Err(e) => return todo!(),
        };
        let data = match outcome.ritchie_kind() {
            RitchieKind::Type(RitchieTypeKind::Fn) => SemaExprData::FunctionFnCall {
                function_sema_expr_idx,
                template_arguments: template_arguments.map(|_| todo!()),
                lpar_regional_token_idx,
                ritchie_parameter_argument_matches,
                rpar_regional_token_idx,
            },
            RitchieKind::Type(_) => todo!(),
            RitchieKind::Trait(_) => todo!(),
        };
        (Ok(data), Ok(outcome.return_ty()))
    }
}
