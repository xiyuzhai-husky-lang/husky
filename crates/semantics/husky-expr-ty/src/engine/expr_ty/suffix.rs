mod unveil;
mod unwrap;

pub(crate) use self::unveil::*;

use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_suffix_expr_ty(
        &mut self,
        expr_idx: ExprIdx,
        opd: ExprIdx,
        opr: SuffixOpr,
        final_destination: FinalDestination,
    ) -> ExprTypeResult<(ExprDisambiguation, ExprTypeResult<FluffyTerm>)> {
        match opr {
            SuffixOpr::Incr => Ok((ExprDisambiguation::Trivial, self.calc_incr_expr_ty(opd))),
            SuffixOpr::Decr => Ok((ExprDisambiguation::Trivial, self.calc_decr_expr_ty(opd))),
            SuffixOpr::UnveilOrComposeWithOption => {
                self.calc_unveil_or_compose_with_option_expr_ty(opd, final_destination)
            }
            SuffixOpr::UnwrapOrComposeWithNot => self.calc_ambiguous_suffix_expr_ty(
                opd,
                final_destination,
                (
                    UnwrapOrComposeWithNotExprDisambiguation::Unwrap,
                    Self::calc_unwrap_expr_ty,
                ),
                (
                    UnwrapOrComposeWithNotExprDisambiguation::ComposeWithNot,
                    Self::calc_compose_with_option_expr_ty,
                ),
            ),
        }
    }

    fn calc_incr_expr_ty(&mut self, opd: ExprIdx) -> ExprTypeResult<FluffyTerm> {
        let opd_ty = self
            .infer_new_expr_ty(opd, ExpectAnyOriginal)
            .ok_or(DerivedExprTypeError::SuffixOperandTypeNotInferred)?;
        match opd_ty.data(self) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                ty_path: path,
                refined_ty_path: refined_path,
                arguments,
                ty_ethereal_term,
            } => todo!(),
            FluffyTermData::TypeOntologyAtPlace {
                place,
                ty_path: path,
                refined_ty_path: refined_path,
                arguments,
                base_ty_ethereal_term,
            } => todo!(),
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
                ..
            } => todo!(),
            FluffyTermData::HoleAtPlace {
                place,
                hole_kind,
                hole,
            } => todo!(),
            FluffyTermData::Symbol { .. } => todo!(),
            FluffyTermData::SymbolAtPlace { .. } => todo!(),
            FluffyTermData::Variable { ty } => todo!(),
            FluffyTermData::TypeVariant { path } => todo!(),
        }
        Ok(self.term_menu.unit_ty_ontology().into())
    }

    fn calc_decr_expr_ty(&mut self, opd: ExprIdx) -> ExprTypeResult<FluffyTerm> {
        let opd_ty = self
            .infer_new_expr_ty(opd, ExpectAnyOriginal)
            .ok_or(DerivedExprTypeError::SuffixOperandTypeNotInferred)?;
        match opd_ty.data(self) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                ty_path: path,
                refined_ty_path: refined_path,
                arguments,
                ty_ethereal_term,
            } => todo!(),
            FluffyTermData::TypeOntologyAtPlace {
                place,
                ty_path: path,
                refined_ty_path: refined_path,
                arguments,
                base_ty_ethereal_term,
            } => todo!(),
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
                ..
            } => todo!(),
            FluffyTermData::HoleAtPlace {
                place,
                hole_kind,
                hole,
            } => todo!(),
            FluffyTermData::Symbol { .. } => todo!(),
            FluffyTermData::SymbolAtPlace { .. } => todo!(),
            FluffyTermData::Variable { ty } => todo!(),
            FluffyTermData::TypeVariant { path } => todo!(),
        }
        Ok(self.term_menu.unit_ty_ontology().into())
    }

    fn calc_unveil_or_compose_with_option_expr_ty(
        &mut self,
        opd: ExprIdx,
        expected_final_destination: FinalDestination,
    ) -> ExprTypeResult<(ExprDisambiguation, ExprTypeResult<FluffyTerm>)> {
        match self.unveiler {
            Unveiler::Unique {
                opd_ty,
                unveil_output_ty,
                unveil_output_ty_final_destination,
            } => match unveil_output_ty_final_destination {
                FinalDestination::Sort => todo!(),
                FinalDestination::TypeOntology => match expected_final_destination {
                    FinalDestination::Sort => todo!(),
                    FinalDestination::TypeOntology => {
                        self.infer_new_expr_ty_discarded(
                            opd,
                            ExpectCoersion::new(Contract::Move, opd_ty.into()),
                        );
                        Ok((
                            ExprDisambiguation::UnveilOrComposeWithOption(
                                UnveilOrComposeWithOptionExprDisambiguation::Unveil,
                            ),
                            Ok(unveil_output_ty.into()),
                        ))
                    }
                    FinalDestination::AnyOriginal => todo!(),
                    FinalDestination::AnyDerived => todo!(),
                },
                FinalDestination::AnyOriginal => todo!(),
                FinalDestination::AnyDerived => todo!(),
            },
            Unveiler::Nothing => todo!(),
            Unveiler::ErrUnableToInferReturnTypeForUnveiling
            | Unveiler::ErrEtherealSignature(_) => {
                self.infer_new_expr_ty_discarded(opd, ExpectAnyDerived);
                Err(DerivedExprTypeError::UnveilerError)?
            }
        }
    }

    fn calc_ambiguous_suffix_expr_ty<D: std::fmt::Debug + Into<ExprDisambiguation>>(
        &mut self,
        opd: ExprIdx,
        final_destination: FinalDestination,
        (true_suffix, true_suffix_f): (
            D,
            fn(&mut Self, opd_ty: FluffyTerm) -> ExprTypeResult<FluffyTerm>,
        ),
        (application_composition, application_composition_f): (
            D,
            fn(&mut Self, opd_ty: FluffyTerm) -> ExprTypeResult<FluffyTerm>,
        ),
    ) -> ExprTypeResult<(ExprDisambiguation, ExprTypeResult<FluffyTerm>)> {
        match self.infer_new_expr_ty(opd, ExpectFinalDestination::new(final_destination)) {
            Some(opd_ty) => match opd_ty.data(self) {
                FluffyTermData::Literal(_) => todo!(),
                FluffyTermData::TypeOntology { .. }
                | FluffyTermData::TypeOntologyAtPlace { .. } => {
                    Ok((true_suffix.into(), true_suffix_f(self, opd_ty)))
                }
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
                FluffyTermData::HoleAtPlace {
                    place,
                    hole_kind,
                    hole,
                } => todo!(),
                FluffyTermData::Symbol { .. } => todo!(),
                FluffyTermData::SymbolAtPlace { .. } => todo!(),
                FluffyTermData::Variable { ty } => todo!(),
                FluffyTermData::TypeVariant { path } => todo!(),
            },
            None => Err(DerivedExprTypeError::UnableToInferSuffixOperandType.into()),
        }
    }

    fn calc_compose_with_option_expr_ty(
        &mut self,
        opd_ty: FluffyTerm,
    ) -> ExprTypeResult<FluffyTerm> {
        match opd_ty.data(self) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                ty_path: path,
                refined_ty_path: refined_path,
                arguments,
                ty_ethereal_term,
            } => todo!(),
            FluffyTermData::TypeOntologyAtPlace {
                place,
                ty_path,
                refined_ty_path,
                arguments,
                base_ty_ethereal_term,
            } => todo!(),
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
                ..
            } => todo!(),
            FluffyTermData::HoleAtPlace {
                place,
                hole_kind,
                hole,
            } => todo!(),
            FluffyTermData::Symbol { .. } => todo!(),
            FluffyTermData::SymbolAtPlace { .. } => todo!(),
            FluffyTermData::Variable { ty } => todo!(),
            FluffyTermData::TypeVariant { path } => todo!(),
        }
    }
}
