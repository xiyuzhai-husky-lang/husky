use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_ritchie_call_nonself_arguments_expr_ty(
        &mut self,
        expr_idx: ExprIdx,
        nonself_parameter_liasoned_tys: &[LocalTermRitchieParameterLiasonedType],
        nonself_arguments: ExprIdxRange,
        local_term_region: &mut LocalTermRegion,
    ) {
        if nonself_parameter_liasoned_tys.len() != nonself_arguments.len() {
            self.add_expr_ty_error(
                expr_idx,
                OriginalExprTypeError::RitchieCallWrongNumberOfArguments {
                    number_of_nonself_parameters: nonself_parameter_liasoned_tys.len() as u8,
                    number_of_nonself_arguments: nonself_arguments.len() as u8,
                },
            )
        }
        for (i, nonself_argument) in nonself_arguments.into_iter().enumerate() {
            if i < nonself_parameter_liasoned_tys.len() {
                let nonself_parameter_liasoned_ty = nonself_parameter_liasoned_tys[i];
                self.infer_new_expr_ty_discarded(
                    nonself_argument,
                    ExpectImplicitlyConvertible::new(nonself_parameter_liasoned_ty),
                    local_term_region,
                );
            } else {
                self.infer_new_expr_ty_discarded(
                    nonself_argument,
                    ExpectAnyDerived,
                    local_term_region,
                );
            }
        }
    }
}
