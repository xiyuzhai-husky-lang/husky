use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_ambiguous_suffix_expr_ty<F1, F2, F3>(
        &mut self,
        opd: SynExprIdx,
        final_destination: FinalDestination,
        naive_suffix_f_given_opd_ty: F1,
        naive_suffix_f: F2,
        application_composition_f: F3,
    ) -> ExprTypeResult<(SynExprDisambiguation, ExprTypeResult<FluffyTerm>)>
    where
        F1: FnOnce(
            &mut Self,
            FluffyTerm,
        ) -> ExprTypeResult<(SynExprDisambiguation, ExprTypeResult<FluffyTerm>)>,
        F2: FnOnce(
            &mut Self,
            SynExprIdx,
        ) -> ExprTypeResult<(SynExprDisambiguation, ExprTypeResult<FluffyTerm>)>,
        F3: FnOnce(
            &mut Self,
            SynExprIdx,
            FinalDestination,
        ) -> ExprTypeResult<(SynExprDisambiguation, ExprTypeResult<FluffyTerm>)>,
    {
        match final_destination {
            FinalDestination::Sort => {
                match self.infer_new_expr_ty(opd, ExpectFinalDestination::new(final_destination)) {
                    Some(opd_ty) => match opd_ty.data(self) {
                        FluffyTermData::Literal(_) => todo!(),
                        FluffyTermData::TypeOntology { .. } => {
                            naive_suffix_f_given_opd_ty(self, opd_ty)
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
                    None => Err(DerivedExprTypeError::UnableToInferSuffixOperandType.into()),
                }
            }
            _ => naive_suffix_f(self, opd),
        }
    }
}
