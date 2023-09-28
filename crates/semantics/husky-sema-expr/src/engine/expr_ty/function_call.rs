use super::*;

impl<'a> SemaExprEngine<'a> {
    pub(super) fn calc_function_application_or_call_expr_ty(
        &mut self,
        syn_expr_idx: SynExprIdx,
        function_syn_expr_idx: SynExprIdx,
        expr_ty_expectation: &impl ExpectFluffyTerm,
        generic_arguments: Option<&SynTemplateArgumentList>,
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
        if let Some(generic_arguments) = generic_arguments {
            todo!()
        }
        match outcome.variant() {
            ExpectEqsFunctionTypeOutcomeVariant::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
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
                    Ok(SemaExprData::FnCall {
                        function_sema_expr_idx,
                        template_arguments: todo!(),
                        lpar_regional_token_idx,
                        ritchie_parameter_argument_matches,
                        rpar_regional_token_idx,
                    }),
                    Ok(outcome.return_ty()),
                )
            }
            ExpectEqsFunctionTypeOutcomeVariant::Curry {
                variance,
                parameter_symbol,
                parameter_ty,
                return_ty,
            } => {
                let argument_sema_expr_idx = match items.len() {
                    0 => unreachable!(),
                    1 => self.build_sema_expr(
                        items.first().expect("len is 1").syn_expr_idx(),
                        ExpectCoersion::new_const(*parameter_ty),
                    ),
                    // parameter_ty must be a tuple
                    // distribute the types for a tuple
                    _ => todo!(),
                };
                (
                    Ok(SemaExprData::Application {
                        function_sema_expr_idx,
                        argument_sema_expr_idx,
                    }),
                    Ok(*return_ty),
                )
            }
        }
    }

    pub(super) fn calc_function_call_expr_ty(
        &mut self,
        expr_idx: SynExprIdx,
        function: SynExprIdx,
        final_destination: FinalDestination,
        generic_arguments: Option<&SynTemplateArgumentList>,
        items: &[SynCallListItem],
    ) -> (
        SemaExprDataResult<SemaExprData>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        let (function_sema_expr_idx, outcome) = self
            .build_sema_expr_with_outcome(function, ExpectEqsRitchieType::new(final_destination));
        let Some(outcome) = outcome else {
            for item in items {
                self.build_sema_expr_with_its_ty_returned(
                    item.argument_expr_idx(),
                    ExpectAnyDerived,
                );
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
            RitchieKind::FnType => SemaExprData::FnCall {
                function_sema_expr_idx: todo!(),
                template_arguments: todo!(),
                lpar_regional_token_idx: todo!(),
                ritchie_parameter_argument_matches,
                rpar_regional_token_idx: todo!(),
            },
            RitchieKind::FnTrait => todo!(),
            RitchieKind::FnMutTrait => todo!(),
            RitchieKind::GnType => todo!(),
        };
        (Ok(data), Ok(outcome.return_ty()))
    }
}
