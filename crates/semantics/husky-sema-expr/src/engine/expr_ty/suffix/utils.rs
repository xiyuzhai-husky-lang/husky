use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_ambiguous_suffix_expr_ty<F1, F2, F3>(
        &mut self,
        opd: SynExprIdx,
        final_destination: FinalDestination,
        naive_suffix_f_given_opd_ty: F1,
        naive_suffix_f: F2,
        application_composition_f: F3,
    ) -> (SemaExprIdx, SemaExprTypeResult<FluffyTerm>)
    where
        F1: FnOnce(
            &mut Self,
            FluffyTerm,
        ) -> (
            SemaExprDataResult<SemaExprData>,
            SemaExprTypeResult<FluffyTerm>,
        ),
        F2: FnOnce(
            &mut Self,
            SynExprIdx,
        ) -> (
            SemaExprDataResult<SemaExprData>,
            SemaExprTypeResult<FluffyTerm>,
        ),
        F3: FnOnce(
            &mut Self,
            SynExprIdx,
            FinalDestination,
        ) -> (
            SemaExprDataResult<SemaExprData>,
            SemaExprTypeResult<FluffyTerm>,
        ),
    {
        match final_destination {
            FinalDestination::Sort => {
                let (opd_sema_expr_idx, opd_ty) =
                    self.build_new_expr_ty(opd, ExpectFinalDestination::new(final_destination));
                match opd_ty {
                    Some(opd_ty) => match opd_ty.data(self) {
                        FluffyTermData::Literal(_) => todo!(),
                        FluffyTermData::TypeOntology { .. } => {
                            (opd_sema_expr_idx, naive_suffix_f_given_opd_ty(self, opd_ty))
                        }
                        FluffyTermData::Curry { .. } => todo!(),
                        FluffyTermData::Hole(_, _) => todo!(),
                        FluffyTermData::Category(_) => todo!(),
                        FluffyTermData::Ritchie {
                            ritchie_kind,
                            parameter_contracted_tys,
                            return_ty,
                        } => todo!(),
                        FluffyTermData::Symbol { .. } => todo!(),
                        FluffyTermData::Variable { ty } => todo!(),
                        FluffyTermData::TypeVariant { path } => todo!(),
                    },
                    None => Err(DerivedSemaExprTypeError::UnableToInferSuffixOperandType.into()),
                }
            }
            _ => naive_suffix_f(self, opd),
        }
    }
}
