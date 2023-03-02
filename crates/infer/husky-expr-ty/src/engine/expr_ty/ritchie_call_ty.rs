use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_ritchie_call_arguments_ty(
        &mut self,
        ritchie_kind: TermRitchieKind,
        nonself_parameter_liasoned_tys: Vec<LocalTermRitchieParameter>,
        nonself_arguments: ExprIdxRange,
        local_term_region: &mut LocalTermRegion,
    ) {
        if nonself_parameter_liasoned_tys.len() != nonself_arguments.len() {
            todo!()
        }
        for (nonself_parameter_liasoned_ty, nonself_argument) in
            std::iter::zip(nonself_parameter_liasoned_tys, nonself_arguments)
        {
            self.infer_new_expr_ty(
                nonself_argument,
                ExpectImplicitlyConvertible {
                    destination: nonself_parameter_liasoned_ty.ty,
                },
                local_term_region,
            );
        }
    }
}
