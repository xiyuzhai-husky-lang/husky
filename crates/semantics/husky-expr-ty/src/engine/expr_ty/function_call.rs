use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_function_application_or_call_expr_ty(
        &mut self,
        expr_idx: SynExprIdx,
        function: SynExprIdx,
        expr_ty_expectation: &impl ExpectFluffyTerm,
        generic_arguments: Option<&SynTemplateArgumentList>,
        items: &[SynCommaListItem],
    ) -> ExprTypeResult<(SynExprDisambiguation, ExprTypeResult<FluffyTerm>)> {
        let Some(outcome) = self.infer_new_expr_ty_for_outcome(
            function,
            ExpectEqsFunctionType::new(expr_ty_expectation.final_destination(self)),
        ) else {
            for item in items {
                self.infer_new_expr_ty(item.expr_idx(), ExpectAnyDerived);
            }
            Err(DerivedExprTypeError::ApplicationOrRitchieCallFunctionTypeNotInferred)?
        };
        if let Some(generic_arguments) = generic_arguments {
            todo!()
        }
        match outcome.variant() {
            ExpectEqsFunctionTypeOutcomeVariant::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
            } => {
                let ritchie_parameter_argument_matches = self.calc_ritchie_arguments_ty(
                    expr_idx,
                    parameter_contracted_tys,
                    items.iter().copied().map(Into::into),
                )?;
                Ok((
                    SynExprDisambiguation::ApplicationOrFunctionCall(
                        ApplicationOrFunctionCallExprDisambiguation::FnCall {
                            ritchie_parameter_argument_matches,
                        },
                    ),
                    Ok(outcome.return_ty()),
                ))
            }
            ExpectEqsFunctionTypeOutcomeVariant::Curry {
                variance,
                parameter_symbol,
                parameter_ty,
                return_ty,
            } => {
                match items.len() {
                    0 => unreachable!(),
                    1 => self.infer_new_expr_ty_discarded(
                        items.first().expect("len is 1").expr_idx(),
                        ExpectCoersion::new_const(*parameter_ty),
                    ),
                    // parameter_ty must be a tuple
                    // distribute the types for a tuple
                    _ => todo!(),
                }
                Ok((
                    SynExprDisambiguation::ApplicationOrFunctionCall(
                        ApplicationOrFunctionCallExprDisambiguation::Application,
                    ),
                    Ok(*return_ty),
                ))
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
    ) -> ExprTypeResult<(SynExprDisambiguation, ExprTypeResult<FluffyTerm>)> {
        let Some(outcome) = self
            .infer_new_expr_ty_for_outcome(function, ExpectEqsRitchieType::new(final_destination))
        else {
            for item in items {
                self.infer_new_expr_ty(item.argument_expr_idx(), ExpectAnyDerived);
            }
            Err(DerivedExprTypeError::ApplicationOrRitchieCallFunctionTypeNotInferred)?
        };
        let ritchie_parameter_argument_matches = self.calc_ritchie_arguments_ty(
            expr_idx,
            outcome.parameter_contracted_tys(),
            items.iter().copied(),
        );
        Ok((
            SynExprDisambiguation::FunctionCall {
                ritchie_kind: outcome.ritchie_kind(),
                ritchie_parameter_argument_matches,
            },
            Ok(outcome.return_ty()),
        ))
    }
}
