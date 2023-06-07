use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_function_application_expr_ty(
        &mut self,
        function_expr_idx: ExprIdx,
        argument_expr_idx: ExprIdx,
        final_destination: FinalDestination,
    ) -> ExprTypeResult<FluffyTerm> {
        let Some(function_ty_outcome) = self.infer_new_expr_ty_for_outcome(
            function_expr_idx,
            ExpectEqsFunctionType::new(final_destination),
        ) else {
            self.infer_new_expr_ty_discarded(argument_expr_idx, ExpectAnyDerived,  );
            return Err(DerivedExprTypeError::ExplicitApplicationFunctionTypeNotInferred.into())
        };
        match function_ty_outcome.variant() {
            ExpectEqsFunctionTypeOutcomeVariant::Curry {
                variance,
                parameter_symbol,
                parameter_ty,
                return_ty,
            } => self.calc_function_application_expr_ty_aux(
                *variance,
                *parameter_symbol,
                *parameter_ty,
                *return_ty,
                argument_expr_idx,
            ),
            ExpectEqsFunctionTypeOutcomeVariant::Ritchie { .. } => {
                self.infer_new_expr_ty_discarded(argument_expr_idx, ExpectAnyDerived);
                Err(OriginalExprTypeError::ExpectedCurryButGotRitchieInstead.into())
            }
        }
    }

    pub(super) fn calc_function_application_expr_ty_aux(
        &mut self,
        variance: Variance,
        parameter_symbol: Option<FluffyTerm>,
        parameter_ty: FluffyTerm,
        return_ty: FluffyTerm,
        argument: ExprIdx,
    ) -> ExprTypeResult<FluffyTerm> {
        let Some(argument_ty) = self.infer_new_expr_ty (
            argument,
            ExpectCurryDestination::new(parameter_ty),
        ) else {
            Err(DerivedExprTypeError::UnableToInferFunctionApplicationArgumentType)?
        };
        let shift =
            argument_ty.curry_parameter_count(self) - parameter_ty.curry_parameter_count(self);
        // needs also to check type
        match shift {
            0 => match parameter_symbol {
                Some(_) => todo!(),
                None => Ok(return_ty),
            },
            shift if shift < 0 => todo!("invalid"),
            shift => self
                .synthesize_function_application_expr_ty(
                    variance,
                    parameter_symbol,
                    parameter_ty,
                    return_ty,
                    argument_ty,
                    shift,
                )
                .map_err(Into::into),
        }
    }
}
