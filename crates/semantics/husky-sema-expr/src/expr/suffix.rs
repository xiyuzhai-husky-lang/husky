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
        SemaExprTypeResult<FlyTerm>,
    ) {
        match opr {
            SynSuffixOpr::Incr => {
                let (opd_sema_expr_idx, opd_ty) =
                    self.build_sema_expr_with_ty(opd_syn_expr_idx, ExpectNumType);
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
                    self.build_sema_expr_with_ty(opd_syn_expr_idx, ExpectNumType);
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

    fn calc_incr_or_decr_expr_ty(&mut self, _: Option<FlyTerm>) -> SemaExprTypeResult<FlyTerm> {
        Ok(self.term_menu().unit_ty_ontology().into())
    }
}
