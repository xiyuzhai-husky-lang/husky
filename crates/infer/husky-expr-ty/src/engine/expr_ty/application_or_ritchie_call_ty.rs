use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_application_or_ritchie_call_expr(
        &mut self,
        function: idx_arena::ArenaIdx<Expr>,
        expr_ty_expectation: &impl ExpectLocalTerm,
        local_term_region: &mut LocalTermRegion,
        implicit_arguments: &Option<ImplicitArgumentList>,
        items: &idx_arena::ArenaIdxRange<Expr>,
    ) -> (
        ExprTypeResult<LocalTerm>,
        ExprTypeResult<ExprDisambiguation>,
    ) {
        let Some(expectation_ok) = self.infer_new_expr_ty_with_expectation_returned(
                function,
                ExpectEqsFunctionType::new(expr_ty_expectation.destination()),
                local_term_region,
            ) else {
                for item in items {
                    self.infer_new_expr_ty(item, ExpectAnyDerived, local_term_region);
                }
                return (
                    Err(
                        DerivedExprTypeError::ApplicationOrRitchieCallFunctionTypeNotInferred
                            .into(),
                    ),
                    Err(
                        DerivedExprTypeError::ApplicationOrRitchieCallFunctionTypeNotInferred
                            .into(),
                    ),
                )
            };
        if let Some(implicit_arguments) = implicit_arguments {
            todo!()
        }
        match expectation_ok.variant() {
            ExpectEqsFunctionTypeOkVariant::Ritchie {
                ritchie_kind,
                parameter_liasoned_tys,
            } => {
                self.calc_ritchie_call_arguments_ty(
                    *ritchie_kind,
                    parameter_liasoned_tys.to_vec(),
                    *items,
                    local_term_region,
                );
                (
                    Ok(expectation_ok.return_ty()),
                    Ok(ExprDisambiguation::ApplicationOrRitchieCall(
                        ApplicationOrRitchieCallExprDisambiguation::RitchieCall,
                    )),
                )
            }
            ExpectEqsFunctionTypeOkVariant::Curry {} => todo!(),
        }
    }
}
