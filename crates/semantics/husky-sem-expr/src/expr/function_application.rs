use super::*;

/// # type

impl<'a> SemExprBuilder<'a> {
    pub(super) fn calc_function_application_expr_ty(
        &mut self,
        expr_idx: SynExprIdx,
        function_syn_expr_idx: SynExprIdx,
        argument_syn_expr_idx: SynExprIdx,
        final_destination: FinalDestination,
    ) -> (SemExprDataResult<SemExprData>, SemExprTypeResult<FlyTerm>) {
        let (function_sem_expr_idx, function_ty_outcome) = self.build_sem_expr_with_outcome(
            function_syn_expr_idx,
            ExpectEqsFunctionType::new(final_destination),
        );
        let Some(function_ty_outcome) = function_ty_outcome else {
            return (
                Err(DerivedSemExprDataError::ExplicitApplicationFunctionTypeNotInferred.into()),
                Err(DerivedSemExprTypeError::ExplicitApplicationFunctionTypeNotInferred.into()),
            );
        };
        match function_ty_outcome.variant() {
            ExpectEqsFunctionTypeOutcomeData::ExplicitCurry {
                variance,
                parameter_hvar,
                parameter_ty,
                return_ty,
            } => {
                let (argument_sem_expr_idx, ty_result) = self
                    .calc_function_application_expr_ty_aux(
                        expr_idx,
                        *variance,
                        *parameter_hvar,
                        *parameter_ty,
                        *return_ty,
                        argument_syn_expr_idx,
                    );
                (
                    Ok(SemExprData::FunctionApplication {
                        function: function_sem_expr_idx,
                        argument: argument_sem_expr_idx,
                    }),
                    ty_result,
                )
            }
            ExpectEqsFunctionTypeOutcomeData::TypeRitchie { .. } => {
                let argument_sem_expr_idx =
                    self.build_sem_expr(argument_syn_expr_idx, ExpectAnyDerived);
                (
                    Ok(SemExprData::FunctionApplication {
                        function: function_sem_expr_idx,
                        argument: argument_sem_expr_idx,
                    }),
                    Err(OriginalSemExprTypeError::ExpectedCurryButGotRitchieInstead.into()),
                )
            }
        }
    }

    /// returns (argument_sem_expr_idx, ty_result)
    pub(super) fn calc_function_application_expr_ty_aux(
        &mut self,
        syn_expr_idx: SynExprIdx,
        variance: Variance,
        parameter_hvar: Option<FlyHvar>,
        parameter_ty: FlyTerm,
        return_ty: FlyTerm,
        argument_syn_expr_idx: SynExprIdx,
    ) -> (SemExprIdx, SemExprTypeResult<FlyTerm>) {
        let (argument_sem_expr_idx, argument_ty) = self.build_sem_expr_with_ty(
            argument_syn_expr_idx,
            ExpectCurryDestination::new(parameter_ty),
        );
        let Some(argument_ty) = argument_ty else {
            return (
                argument_sem_expr_idx,
                Err(DerivedSemExprTypeError::UnableToInferFunctionApplicationArgumentType.into()),
            );
        };
        let shift =
            argument_ty.curry_parameter_count(self) - parameter_ty.curry_parameter_count(self);
        // needs also to check type
        let ty_result = match shift {
            0 => match parameter_hvar {
                Some(parameter_hvar) => match self.infer_expr_term(argument_sem_expr_idx) {
                    Some(argument_term) => Ok(return_ty.substitute_hvar(
                        self,
                        syn_expr_idx.into(),
                        parameter_hvar,
                        argument_term,
                    )),
                    None => Err(
                        DerivedSemExprTypeError::UnableToInferArgumentTermForDependentType.into(),
                    ),
                },
                None => Ok(return_ty),
            },
            shift if shift < 0 => todo!("invalid"),
            shift => self
                .synthesize_function_application_expr_ty(
                    variance,
                    parameter_hvar,
                    parameter_ty,
                    return_ty,
                    argument_ty,
                    shift,
                )
                .map_err(Into::into),
        };
        return (argument_sem_expr_idx, ty_result);
    }
}

/// # term

impl<'a> SemExprBuilder<'a> {
    pub(crate) fn calc_explicit_application_expr_term(
        &mut self,
        function: SemExprIdx,
        argument: SemExprIdx,
    ) -> SemExprTermResult<FlyTerm> {
        // todo: implicit arguments
        let function = self
            .infer_expr_term(function)
            .ok_or(DerivedSemExprTermError::ExplicitApplicationFunctionTermNotInferred)?;
        let argument = self
            .infer_expr_term(argument)
            .ok_or(DerivedSemExprTermError::ExplicitApplicationArgumentTermNotInferred)?;
        FlyTerm::new_application(self, function, argument)
            .map_err(|e| DerivedSemExprTermError::ExplicitApplicationTerm(e).into())
    }
}
