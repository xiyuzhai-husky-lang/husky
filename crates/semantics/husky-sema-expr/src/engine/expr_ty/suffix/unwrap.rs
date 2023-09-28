use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_unwrap_expr_ty_given_opd_ty(
        &mut self,
        opd_ty: FluffyTerm,
    ) -> (
        SemaExprDataResult<SemaSuffixOpr>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        todo!()
    }

    pub(super) fn calc_unwrap_expr_ty(
        &mut self,
        opd: SynExprIdx,
    ) -> (
        SemaExprDataResult<(SemaExprIdx, SemaSuffixOpr)>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        let (opd_sema_expr_idx, opd_ty) = self.build_new_expr_ty(opd, ExpectAnyOriginal);
        let Some(opd_ty) = opd_ty else {
            // p!(self.expr_region_data.path().debug(self.db));
            // todo!();
            Err(DerivedSemaExprTypeError::UnableToInferUnwrapOperand)?
        };
        match opd_ty.data(self) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                ty_path,
                refined_ty_path,
                ty_arguments,
                ty_ethereal_term,
            } => match refined_ty_path {
                Left(PreludeTypePath::Option | PreludeTypePath::Result) => {
                    (Ok(SemaExprData::Unwrap.into()), Ok(ty_arguments[0]))
                }
                _ => return (todo!(), Err(OriginalSemaExprTypeError::CannotUnwrap.into())),
            },
            FluffyTermData::Curry {
                curry_kind,
                variance,
                parameter_variable,
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
            } => todo!(),
            FluffyTermData::Symbol { term, ty } => todo!(),
            FluffyTermData::Variable { ty } => todo!(),
            FluffyTermData::TypeVariant { path } => todo!(),
        }
    }
}
