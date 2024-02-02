use super::*;
use husky_sema_opr::suffix::SemaSuffixOpr;

impl<'a> SemaExprEngine<'a> {
    pub(super) fn calc_compose_with_option_expr_ty(
        &mut self,
        opd: SynExprIdx,
        final_destination: FinalDestination,
    ) -> (
        SemaExprDataResult<SemaExprData>,
        SemaExprTypeResult<FlyTerm>,
    ) {
        todo!()
    }

    pub(super) fn calc_compose_with_option_expr_ty_given_opd_ty(
        &mut self,
        opd_ty: FlyTerm,
    ) -> (
        SemaExprDataResult<SemaSuffixOpr>,
        SemaExprTypeResult<FlyTerm>,
    ) {
        match opd_ty.data(self) {
            FlyTermData::Literal(_) => todo!(),
            FlyTermData::TypeOntology {
                ty_path: path,
                refined_ty_path: refined_path,
                ty_arguments: arguments,
                ty_ethereal_term,
            } => todo!(),
            FlyTermData::Curry {
                toolchain,
                curry_kind,
                variance,
                parameter_rune,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => todo!(),
            FlyTermData::Hole(_, _) => todo!(),
            FlyTermData::Category(_) => todo!(),
            FlyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
                ..
            } => todo!(),
            FlyTermData::Symbol { .. } => todo!(),
            FlyTermData::Rune { .. } => todo!(),
            FlyTermData::TypeVariant { path } => todo!(),
        }
    }
}
