use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_explicit_application_expr_ty(
        &mut self,
        function: ExprIdx,
        argument: ExprIdx,
        final_destination: FinalDestination,
        local_term_region: &mut LocalTermRegion,
    ) -> Result<LocalTerm, ExprTypeError> {
        match self.infer_new_expr_ty_with_expectation_returned(
            function,
            ExpectEqsFunctionType::new(final_destination),
            local_term_region,
        ) {
            Some(function_ty_outcome) => match function_ty_outcome.variant {
                ExpectEqsFunctionTypeOutcomeVariant::Curry {
                    parameter_symbol,
                    parameter_ty,
                    return_ty,
                } => {
                    self.infer_new_expr_ty_discarded(
                        argument,
                        ExpectImplicitlyConvertible {
                            destination: parameter_ty,
                        },
                        local_term_region,
                    );
                    match parameter_symbol {
                        Some(_) => todo!(),
                        None => Ok(return_ty),
                    }
                }
                ExpectEqsFunctionTypeOutcomeVariant::Ritchie { .. } => {
                    self.infer_new_expr_ty_discarded(argument, ExpectAnyDerived, local_term_region);
                    Err(todo!("expect curry"))
                }
            },
            None => {
                self.infer_new_expr_ty_discarded(argument, ExpectAnyDerived, local_term_region);
                Err(todo!())
            }
        }
    }
}
