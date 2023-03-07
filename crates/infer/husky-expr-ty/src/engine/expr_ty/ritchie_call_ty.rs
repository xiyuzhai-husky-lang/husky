use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_ritchie_call_arguments_expr_ty(
        &mut self,
        expr_idx: ExprIdx,
        ritchie_kind: TermRitchieKind,
        nonself_parameter_liasoned_tys: Vec<LocalTermRitchieParameter>,
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
                self.infer_new_expr_ty(
                    nonself_argument,
                    ExpectImplicitlyConvertible {
                        destination: nonself_parameter_liasoned_ty.ty,
                    },
                    local_term_region,
                );
            } else {
                self.infer_new_expr_ty(nonself_argument, ExpectAnyDerived, local_term_region);
            }
        }
    }
}
