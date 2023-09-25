mod compose_with_not;
mod compose_with_option;
mod unveil;
mod unwrap;
mod utils;

pub(crate) use self::unveil::*;

use super::*;
use maybe_result::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_suffix_expr_ty(
        &mut self,
        expr_idx: SynExprIdx,
        opd: SynExprIdx,
        opr: SuffixOpr,
        final_destination: FinalDestination,
    ) -> SemaExprResult<(SynExprDisambiguation, SemaExprResult<FluffyTerm>)> {
        match opr {
            SuffixOpr::Incr | SuffixOpr::Attr => Ok((
                SynExprDisambiguation::Trivial,
                self.calc_incr_or_attr_expr_ty(opd),
            )),
            SuffixOpr::UnveilOrComposeWithOption => {
                // self.calc_unveil_or_compose_with_option_expr_ty(opd, final_destination)
                self.calc_ambiguous_suffix_expr_ty(
                    opd,
                    final_destination,
                    Self::calc_unveil_expr_ty_given_opd_ty,
                    Self::calc_unveil_expr_ty,
                    Self::calc_compose_with_option_expr_ty,
                    // Self::calc_compose_with_option_expr_ty_give_opd_ty,
                )
            }
            SuffixOpr::UnwrapOrComposeWithNot => self.calc_ambiguous_suffix_expr_ty(
                opd,
                final_destination,
                Self::calc_unwrap_expr_ty_given_opd_ty,
                Self::calc_unwrap_expr_ty,
                Self::calc_compose_with_not_expr_ty,
            ),
        }
    }

    fn calc_incr_or_attr_expr_ty(&mut self, opd: SynExprIdx) -> SemaExprResult<FluffyTerm> {
        let opd_ty = self
            .infer_new_expr_ty(opd, ExpectAnyOriginal)
            .ok_or(DerivedSemaExprError::SuffixOperandTypeNotInferred)?;
        match opd_ty.data(self) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                ty_path,
                refined_ty_path,
                ty_arguments,
                ty_ethereal_term,
            } => match refined_ty_path {
                Left(PreludeTypePath::Num(_)) => (),
                _ => todo!(),
            },
            FluffyTermData::Curry {
                curry_kind,
                variance,
                parameter_variable,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => todo!(),
            FluffyTermData::Hole(hole_kind, _) => match hole_kind {
                HoleKind::UnspecifiedIntegerType => (),
                HoleKind::UnspecifiedFloatType => todo!(),
                HoleKind::ImplicitType => todo!(),
                HoleKind::Any => todo!(),
            },
            FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
                ..
            } => todo!(),
            FluffyTermData::Symbol { .. } => todo!(),
            FluffyTermData::Variable { ty } => todo!(),
            FluffyTermData::TypeVariant { path } => todo!(),
        }
        Ok(self.term_menu.unit_ty_ontology().into())
    }

    // fn calc_unveil_or_compose_with_option_expr_ty(
    //     &mut self,
    //     opd: SynExprIdx,
    //     expected_final_destination: FinalDestination,
    // ) -> SemaExprResult<(SynExprDisambiguation, SemaExprResult<FluffyTerm>)> {
    //     self.unveiler.initialize_if_not(self.return_ty, self.db);
    //     match self.unveiler {
    //         Unveiler::UniqueFullyInstantiated {
    //             opd_ty,
    //             unveil_output_ty,
    //             unveil_output_ty_final_destination,
    //         } => match unveil_output_ty_final_destination {
    //             FinalDestination::Sort => todo!(),
    //             FinalDestination::TypeOntology => match expected_final_destination {
    //                 FinalDestination::Sort => todo!(),
    //                 FinalDestination::TypeOntology => {
    //                     self.infer_new_expr_ty_discarded(
    //                         opd,
    //                         ExpectCoersion::new(Contract::Move, opd_ty.into()),
    //                     );
    //                     Ok((
    //                         SynExprDisambiguation::UnveilOrComposeWithOption(
    //                             UnveilOrComposeWithOptionExprDisambiguation::Unveil,
    //                         ),
    //                         Ok(unveil_output_ty.into()),
    //                     ))
    //                 }
    //                 FinalDestination::AnyOriginal => todo!(),
    //                 FinalDestination::AnyDerived => todo!(),
    //                 FinalDestination::Ritchie(_) => todo!(),
    //             },
    //             FinalDestination::AnyOriginal => todo!(),
    //             FinalDestination::AnyDerived => todo!(),
    //             FinalDestination::Ritchie(_) => todo!(),
    //         },
    //         Unveiler::UniquePartiallyInstanted { template } => {
    //             let Some(opd_ty) = self.infer_new_expr_ty(opd, ExpectAnyOriginal) else {
    //                 todo!()
    //             };
    //             match opd_ty.base_ty_data(self) {
    //                 FluffyBaseTypeData::TypeOntology {
    //                     ty_path,
    //                     refined_ty_path: Left(PreludeTypePath::Indirection(_)),
    //                     ty_arguments,
    //                     ty_ethereal_term,
    //                 } => todo!(),
    //                 FluffyBaseTypeData::TypeOntology {
    //                     ty_path,
    //                     refined_ty_path,
    //                     ty_arguments,
    //                     ty_ethereal_term,
    //                 } => todo!(),
    //                 FluffyBaseTypeData::Curry {
    //                     curry_kind,
    //                     variance,
    //                     parameter_variable,
    //                     parameter_ty,
    //                     return_ty,
    //                     ty_ethereal_term,
    //                 } => todo!(),
    //                 FluffyBaseTypeData::Hole(_, _) => todo!(),
    //                 FluffyBaseTypeData::Category(_) => todo!(),
    //                 FluffyBaseTypeData::Ritchie {
    //                     ritchie_kind,
    //                     parameter_contracted_tys,
    //                     return_ty,
    //                 } => todo!(),
    //                 FluffyBaseTypeData::Symbol { term } => todo!(),
    //             }
    //             // match opd_ty.base_resolved(self) {
    //             //     FluffyTermBase::Ethereal(opd_ty) => {
    //             //         match template.instantiate_trai(&[opd_ty], self.db) {
    //             //             JustOk(_) => todo!(),
    //             //             JustErr(_) => todo!(),
    //             //             Nothing => todo!(),
    //             //         }
    //             //     }
    //             //     FluffyTermBase::Solid(_) => todo!(),
    //             //     FluffyTermBase::Hollow(_) => todo!(),
    //             // }
    //         }
    //         Unveiler::Nothing => todo!(),
    //         Unveiler::ErrUnableToInferReturnTypeForUnveiling
    //         | Unveiler::ErrEtherealSignature(_) => {
    //             self.infer_new_expr_ty_discarded(opd, ExpectAnyDerived);
    //             Err(DerivedSemaExprError::UnveilerError)?
    //         }
    //         Unveiler::Uninitialized => todo!(),
    //     }
    // }
}
