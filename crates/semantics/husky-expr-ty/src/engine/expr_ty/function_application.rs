use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_function_application_expr_ty(
        &mut self,
        function: ExprIdx,
        argument: ExprIdx,
        final_destination: FinalDestination,
    ) -> ExprTypeResult<FluffyTerm> {
        let Some(function_ty_outcome) = self.infer_new_expr_ty_for_outcome(
            function,
            ExpectEqsFunctionType::new(final_destination),
        ) else {
            self.infer_new_expr_ty_discarded(argument, ExpectAnyDerived,  );
            return Err(DerivedExprTypeError::ExplicitApplicationFunctionTypeNotInferred.into())
        };
        match function_ty_outcome.variant() {
            ExpectEqsFunctionTypeOutcomeVariant::Curry {
                parameter_symbol,
                parameter_ty,
                return_ty,
            } => {
                let Some(argument_ty) = self.infer_new_expr_ty (
                    argument,
                    ExpectAnyTowardsFinalDestination::new(parameter_ty.final_destination(self)),
                ) else {
                    Err(DerivedExprTypeError::UnableToInferFunctionApplicationArgumentType)?
                };
                let shift = argument_ty.curry_parameter_count(self)
                    - parameter_ty.curry_parameter_count(self);
                // needs also to check type
                match shift {
                    0 => match parameter_symbol {
                        Some(_) => todo!(),
                        None => Ok(*return_ty),
                    },
                    shift if shift < 0 => todo!("invalid"),
                    1 => todo!(),
                    _ => todo!(),
                }
            }
            ExpectEqsFunctionTypeOutcomeVariant::Ritchie { .. } => {
                self.infer_new_expr_ty_discarded(argument, ExpectAnyDerived);
                Err(OriginalExprTypeError::ExpectedCurryButGotRitchieInstead.into())
            }
        }
    }
}
