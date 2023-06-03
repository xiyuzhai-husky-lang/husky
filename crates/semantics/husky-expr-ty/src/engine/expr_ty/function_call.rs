use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_explicit_application_or_ritchie_call_expr_ty(
        &mut self,
        expr_idx: ExprIdx,
        function: ExprIdx,
        expr_ty_expectation: &impl ExpectFluffyTerm,
        implicit_arguments: Option<&ImplicitArgumentList>,
        arguments: ExprIdxRange,
    ) -> ExprTypeResult<(ExprDisambiguation, ExprTypeResult<FluffyTerm>)> {
        let Some(expectation_ok) = self.infer_new_expr_ty_for_outcome(
            function,
            ExpectEqsFunctionType::new(expr_ty_expectation.final_destination(self)),
        ) else {
            for item in arguments {
                self.infer_new_expr_ty(item, ExpectAnyDerived);
            }
            Err(DerivedExprTypeError::ApplicationOrRitchieCallFunctionTypeNotInferred)?
        };
        if let Some(implicit_arguments) = implicit_arguments {
            todo!()
        }
        match expectation_ok.variant() {
            ExpectEqsFunctionTypeOutcomeVariant::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
            } => {
                self.calc_ritchie_call_nonself_arguments_expr_ty(
                    expr_idx,
                    parameter_contracted_tys,
                    arguments,
                );
                Ok((
                    ExprDisambiguation::ExplicitApplicationOrFunctionCall(
                        ApplicationOrFunctionCallExprDisambiguation::RitchieCall,
                    ),
                    Ok(expectation_ok.return_ty()),
                ))
            }
            ExpectEqsFunctionTypeOutcomeVariant::Curry {
                parameter_symbol,
                parameter_ty,
                return_ty,
            } => {
                match arguments.len() {
                    0 => unreachable!(),
                    1 => self.infer_new_expr_ty_discarded(
                        arguments.start(),
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

    pub(super) fn calc_ritchie_call_expr_ty(
        &mut self,
        expr_idx: ExprIdx,
        function: ExprIdx,
        final_destination: FinalDestination,
        implicit_arguments: Option<&ImplicitArgumentList>,
        items: &[CallListItem],
    ) -> ExprTypeResult<(ExprDisambiguation, ExprTypeResult<FluffyTerm>)> {
        let Some(expectation_ok) = self.infer_new_expr_ty_for_outcome(
            function,
            ExpectEqsRitchieType::new(final_destination),
        ) else {
            for item in items {
                todo!()
                // self.infer_new_expr_ty(item, ExpectAnyDerived);
            }
            Err(DerivedExprTypeError::ApplicationOrRitchieCallFunctionTypeNotInferred)?
        };
        todo!()
    }
}
