use super::*;

impl<'a> SemaExprEngine<'a> {
    pub(super) fn calc_function_application_expr_ty(
        &mut self,
        expr_idx: SynExprIdx,
        function_syn_expr_idx: SynExprIdx,
        argument_syn_expr_idx: SynExprIdx,
        final_destination: FinalDestination,
    ) -> (
        SemaExprDataResult<SemaExprData>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        let (function_sema_expr_idx, function_ty_outcome) = self.build_sema_expr_with_outcome(
            function_syn_expr_idx,
            ExpectEqsFunctionType::new(final_destination),
        );
        let Some(function_ty_outcome) = function_ty_outcome else {
            let argument_sema_expr_idx =
                self.build_sema_expr(argument_syn_expr_idx, ExpectAnyDerived);
            return (
                Ok(SemaExprData::FunctionApplication {
                    function_sema_expr_idx,
                    argument_sema_expr_idx,
                }),
                Err(DerivedSemaExprTypeError::ExplicitApplicationFunctionTypeNotInferred.into()),
            );
        };
        p!(function_sema_expr_idx
            .data_result(&self.sema_expr_arena)
            .debug(self.db));
        match function_ty_outcome.variant() {
            ExpectEqsFunctionTypeOutcomeData::Curry {
                variance,
                parameter_rune,
                parameter_ty,
                return_ty,
            } => {
                let (argument_sema_expr_idx, ty_result) = self
                    .calc_function_application_expr_ty_aux(
                        expr_idx,
                        *variance,
                        *parameter_rune,
                        *parameter_ty,
                        *return_ty,
                        argument_syn_expr_idx,
                    );
                (
                    Ok(SemaExprData::FunctionApplication {
                        function_sema_expr_idx,
                        argument_sema_expr_idx,
                    }),
                    ty_result,
                )
            }
            ExpectEqsFunctionTypeOutcomeData::Ritchie { .. } => {
                let argument_sema_expr_idx =
                    self.build_sema_expr(argument_syn_expr_idx, ExpectAnyDerived);
                (
                    Ok(SemaExprData::FunctionApplication {
                        function_sema_expr_idx,
                        argument_sema_expr_idx,
                    }),
                    Err(OriginalSemaExprTypeError::ExpectedCurryButGotRitchieInstead.into()),
                )
            }
        }
    }

    /// returns (argument_sema_expr_idx, ty_result)
    pub(super) fn calc_function_application_expr_ty_aux(
        &mut self,
        syn_expr_idx: SynExprIdx,
        variance: Variance,
        parameter_rune: Option<FluffyTermRune>,
        parameter_ty: FluffyTerm,
        return_ty: FluffyTerm,
        argument_syn_expr_idx: SynExprIdx,
    ) -> (SemaExprIdx, SemaExprTypeResult<FluffyTerm>) {
        let (argument_sema_expr_idx, argument_ty) = self.build_sema_expr_with_ty(
            argument_syn_expr_idx,
            ExpectCurryDestination::new(parameter_ty),
        );
        let Some(argument_ty) = argument_ty else {
            return (
                argument_sema_expr_idx,
                Err(DerivedSemaExprTypeError::UnableToInferFunctionApplicationArgumentType.into()),
            );
        };
        let shift =
            argument_ty.curry_parameter_count(self) - parameter_ty.curry_parameter_count(self);
        // needs also to check type
        let ty_result = match shift {
            0 => match parameter_rune {
                Some(parameter_rune) => match self.infer_expr_term(argument_sema_expr_idx) {
                    Some(argument_term) => {
                        p!(parameter_rune.show(self.db, self.fluffy_terms()));
                        p!(return_ty.show(self.db, self.fluffy_terms()));
                        Ok(return_ty.substitute_rune(
                            self,
                            syn_expr_idx.into(),
                            parameter_rune,
                            argument_term,
                        ))
                    }
                    None => Err(
                        DerivedSemaExprTypeError::UnableToInferArgumentTermForDependentType.into(),
                    ),
                },
                None => Ok(return_ty),
            },
            shift if shift < 0 => todo!("invalid"),
            shift => self
                .synthesize_function_application_expr_ty(
                    variance,
                    parameter_rune,
                    parameter_ty,
                    return_ty,
                    argument_ty,
                    shift,
                )
                .map_err(Into::into),
        };
        return (argument_sema_expr_idx, ty_result);
    }
}
