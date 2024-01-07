mod compose_with_not;
mod compose_with_option;
mod unveil;
mod unwrap;
mod utils;

pub(crate) use self::unveil::*;

use super::*;
use husky_sema_opr::suffix::SemaSuffixOpr;
use husky_syn_opr::SynSuffixOpr;
use maybe_result::*;

impl<'a> SemaExprEngine<'a> {
    pub(super) fn calc_suffix_expr_ty(
        &mut self,
        expr_idx: SynExprIdx,
        opd_syn_expr_idx: SynExprIdx,
        opr: SynSuffixOpr,
        opr_regional_token_idx: RegionalTokenIdx,
        final_destination: FinalDestination,
    ) -> (
        SemaExprDataResult<SemaExprData>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        match opr {
            SynSuffixOpr::Incr => {
                let (opd_sema_expr_idx, opd_ty) =
                    self.build_sema_expr_with_ty(opd_syn_expr_idx, ExpectAnyOriginal);
                (
                    Ok(SemaExprData::Suffix {
                        opd_sema_expr_idx,
                        opr: SemaSuffixOpr::Incr,
                        opr_regional_token_idx,
                    }),
                    self.calc_incr_or_decr_expr_ty(opd_ty),
                )
            }
            SynSuffixOpr::Decr => {
                let (opd_sema_expr_idx, opd_ty) =
                    self.build_sema_expr_with_ty(opd_syn_expr_idx, ExpectAnyOriginal);
                (
                    Ok(SemaExprData::Suffix {
                        opd_sema_expr_idx,
                        opr: SemaSuffixOpr::Decr,
                        opr_regional_token_idx,
                    }),
                    self.calc_incr_or_decr_expr_ty(opd_ty),
                )
            }
            SynSuffixOpr::UnveilOrComposeWithOption => self.calc_ambiguous_suffix_expr_ty(
                opd_syn_expr_idx,
                opr_regional_token_idx,
                final_destination,
                Self::calc_unveil_expr_ty_given_opd_ty,
                Self::calc_unveil_expr_ty,
                Self::calc_compose_with_option_expr_ty_given_opd_ty,
            ),
            SynSuffixOpr::UnwrapOrComposeWithNot => self.calc_ambiguous_suffix_expr_ty(
                opd_syn_expr_idx,
                opr_regional_token_idx,
                final_destination,
                Self::calc_unwrap_expr_ty_given_opd_ty,
                Self::calc_unwrap_expr_ty,
                Self::calc_compose_with_not_expr_ty_given_opd_ty,
            ),
        }
    }

    fn calc_incr_or_decr_expr_ty(
        &mut self,
        opd_ty: Option<FluffyTerm>,
    ) -> SemaExprTypeResult<FluffyTerm> {
        let opd_ty = opd_ty.ok_or(DerivedSemaExprTypeError::SuffixOperandTypeNotInferred)?;
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
                parameter_rune: parameter_rune,
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
            FluffyTermData::Rune { .. } => todo!(),
            FluffyTermData::TypeVariant { path } => todo!(),
        }
        Ok(self.term_menu().unit_ty_ontology().into())
    }
}
