use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_function_application_or_call_expr_ty(
        &mut self,
        expr_idx: ExprIdx,
        function: ExprIdx,
        expr_ty_expectation: &impl ExpectFluffyTerm,
        implicit_arguments: Option<&ImplicitArgumentList>,
        items: ExprIdxRange,
        commas: &[TokenIdx],
    ) -> ExprTypeResult<(ExprDisambiguation, ExprTypeResult<FluffyTerm>)> {
        let Some(outcome) = self.infer_new_expr_ty_for_outcome(
            function,
            ExpectEqsFunctionType::new(expr_ty_expectation.final_destination(self)),
        ) else {
            for item in items {
                self.infer_new_expr_ty(item, ExpectAnyDerived);
            }
            Err(DerivedExprTypeError::ApplicationOrRitchieCallFunctionTypeNotInferred)?
        };
        if let Some(implicit_arguments) = implicit_arguments {
            todo!()
        }
        match outcome.variant() {
            ExpectEqsFunctionTypeOutcomeVariant::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
            } => {
                self.calc_ritchie_call_nonself_arguments_expr_ty(
                    expr_idx,
                    parameter_contracted_tys,
                    items
                        .into_iter()
                        .enumerate()
                        .map(|(i, item)| CallListItem::new_regular(item, commas.get(i).copied())),
                );
                Ok((
                    ExprDisambiguation::ExplicitApplicationOrFunctionCall(
                        ApplicationOrFunctionCallExprDisambiguation::RitchieCall,
                    ),
                    Ok(outcome.return_ty()),
                ))
            }
            ExpectEqsFunctionTypeOutcomeVariant::Curry {
                parameter_symbol,
                parameter_ty,
                return_ty,
            } => {
                match items.len() {
                    0 => unreachable!(),
                    1 => self.infer_new_expr_ty_discarded(
                        items.start(),
                        ExpectImplicitlyConvertible::new_const(*parameter_ty),
                    ),
                    // parameter_ty must be a tuple
                    // distribute the types for a tuple
                    _ => todo!(),
                }
                Ok((
                    ExprDisambiguation::ExplicitApplicationOrFunctionCall(
                        ApplicationOrFunctionCallExprDisambiguation::Application,
                    ),
                    Ok(*return_ty),
                ))
            }
        }
    }

    pub(super) fn calc_function_call_expr_ty(
        &mut self,
        expr_idx: ExprIdx,
        function: ExprIdx,
        final_destination: FinalDestination,
        implicit_arguments: Option<&ImplicitArgumentList>,
        items: &[CallListItem],
    ) -> ExprTypeResult<(ExprDisambiguation, ExprTypeResult<FluffyTerm>)> {
        let Some(outcome) = self.infer_new_expr_ty_for_outcome(
            function,
            ExpectEqsRitchieType::new(final_destination),
        ) else {
            for item in items {
                todo!()
                // self.infer_new_expr_ty(item, ExpectAnyDerived);
            }
            Err(DerivedExprTypeError::ApplicationOrRitchieCallFunctionTypeNotInferred)?
        };
        self.calc_ritchie_call_nonself_arguments_expr_ty(
            expr_idx,
            outcome.parameter_contracted_tys(),
            items.iter().copied(),
        );
        Ok((ExprDisambiguation::Trivial, Ok(outcome.return_ty())))
    }
}
