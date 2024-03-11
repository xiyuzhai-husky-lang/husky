use super::*;

impl<'a> SemaExprBuilder<'a> {
    pub(super) fn calc_unwrap_expr_ty_given_opd_ty(
        &mut self,
        opd_ty: FlyTerm,
    ) -> (
        SemaExprDataResult<SemaSuffixOpr>,
        SemaExprTypeResult<FlyTerm>,
    ) {
        todo!()
    }

    pub(super) fn calc_unwrap_expr_ty(
        &mut self,
        opd: SynExprIdx,
        opr_regional_token_idx: RegionalTokenIdx,
    ) -> (
        SemaExprDataResult<SemaExprData>,
        SemaExprTypeResult<FlyTerm>,
    ) {
        let (opd_sema_expr_idx, opd_ty) = self.build_sema_expr_with_ty(opd, ExpectAnyOriginal);
        let Some(opd_ty) = opd_ty else {
            // p!(self.expr_region_data.path().debug(self.db));
            // todo!();
            return (
                Err(todo!()),
                Err(DerivedSemaExprTypeError::UnableToInferUnwrapOperand.into()),
            );
        };
        match opd_ty.data(self) {
            FlyTermData::Literal(_) => todo!(),
            FlyTermData::TypeOntology {
                ty_path,
                refined_ty_path,
                ty_arguments,
                ty_ethereal_term,
            } => match refined_ty_path {
                Left(PreludeTypePath::Option | PreludeTypePath::Result) => (
                    Ok(SemaExprData::Unwrap {
                        opd_sema_expr_idx,
                        opr_regional_token_idx,
                        // unwrap_method_path: todo!(),
                        // instantiation: todo!(),
                    }),
                    Ok(ty_arguments[0]),
                ),
                _ => return (todo!(), Err(OriginalSemaExprTypeError::CannotUnwrap.into())),
            },
            FlyTermData::Curry {
                toolchain,
                curry_kind,
                variance,
                parameter_hvar,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => todo!(),
            FlyTermData::Hole(_, _) => todo!(),
            FlyTermData::Sort(_) => todo!(),
            FlyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => todo!(),
            FlyTermData::Symbol { term, ty } => todo!(),
            FlyTermData::Hvar { .. } => todo!(),
            FlyTermData::TypeVariant { path } => todo!(),
        }
    }
}
