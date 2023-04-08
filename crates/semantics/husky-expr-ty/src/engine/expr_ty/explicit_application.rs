use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_explicit_application_expr_ty(
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
                self.infer_new_expr_ty_discarded(
                    argument,
                    ExpectImplicitlyConvertible::new_transient(*parameter_ty),
                );
                match parameter_symbol {
                    Some(_) => todo!(),
                    None => Ok(*return_ty),
                }
            }
            ExpectEqsFunctionTypeOutcomeVariant::Ritchie { .. } => {
                self.infer_new_expr_ty_discarded(argument, ExpectAnyDerived);
                Err(todo!("expect curry"))
            }
        }
    }
}
