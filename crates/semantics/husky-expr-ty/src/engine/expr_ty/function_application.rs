use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_function_application_expr_ty(
        &mut self,
        expr_idx: ExprIdx,
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
                expr_idx,
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
        expr_idx: ExprIdx,
        variance: Variance,
        parameter_variable: Option<FluffyTerm>,
        parameter_ty: FluffyTerm,
        return_ty: FluffyTerm,
        argument_expr_idx: ExprIdx,
    ) -> ExprTypeResult<FluffyTerm> {
        let Some(argument_ty) = self.infer_new_expr_ty (
            argument_expr_idx,
            ExpectCurryDestination::new(parameter_ty),
        ) else {
            Err(DerivedExprTypeError::UnableToInferFunctionApplicationArgumentType)?
        };
        let shift =
            argument_ty.curry_parameter_count(self) - parameter_ty.curry_parameter_count(self);
        // needs also to check type
        match shift {
            0 => Ok(match parameter_variable {
                Some(parameter_variable) => {
                    let argument_term = self
                        .infer_new_expr_term(argument_expr_idx)
                        .ok_or(DerivedExprTypeError::UnableToInferArgumentTermForDependentType)?;
                    return_ty.substitute_variable(
                        self,
                        expr_idx.into(),
                        parameter_variable,
                        argument_term,
                    )
                }
                None => return_ty,
            }),
            shift if shift < 0 => todo!("invalid"),
            shift => self
                .synthesize_function_application_expr_ty(
                    variance,
                    parameter_variable,
                    parameter_ty,
                    return_ty,
                    argument_ty,
                    shift,
                )
                .map_err(Into::into),
        }
    }
}
