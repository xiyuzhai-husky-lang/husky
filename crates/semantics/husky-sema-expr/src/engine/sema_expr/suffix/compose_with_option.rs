use super::*;
use husky_sema_opr::suffix::SemaSuffixOpr;

impl<'a> SemaExprEngine<'a> {
    pub(super) fn calc_compose_with_option_expr_ty(
        &mut self,
        opd: SynExprIdx,
        final_destination: FinalDestination,
    ) -> (
        SemaExprDataResult<SemaExprData>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        todo!()
    }

    pub(super) fn calc_compose_with_option_expr_ty_given_opd_ty(
        &mut self,
        opd_ty: FluffyTerm,
    ) -> (
        SemaExprDataResult<SemaSuffixOpr>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        match opd_ty.data(self) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                ty_path: path,
                refined_ty_path: refined_path,
                ty_arguments: arguments,
                ty_ethereal_term,
            } => todo!(),
            FluffyTermData::Curry {
                curry_kind,
                variance,
                parameter_rune: parameter_rune,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => todo!(),
            FluffyTermData::Hole(_, _) => todo!(),
            FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
                ..
            } => todo!(),
            FluffyTermData::Symbol { .. } => todo!(),
            FluffyTermData::Rune { ty } => todo!(),
            FluffyTermData::TypeVariant { path } => todo!(),
        }
    }
}
