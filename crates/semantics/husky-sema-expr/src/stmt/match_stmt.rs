use husky_regional_token::{HeavyArrowRegionalToken, VerticalRegionalToken};

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct SemaCaseBranch {
    pub vertical_token: VerticalRegionalToken,
    pub case_pattern_sema_obelisk: CasePatternSemaSyndicate,
    pub heavy_arrow_token: HeavyArrowRegionalToken,
    pub stmts: SemaStmtIdxRange,
}

impl<'a> SemaExprEngine<'a> {
    pub(crate) fn build_match_stmt(
        &mut self,
        match_token: MatchRegionalToken,
        match_target_syn_expr_idx_result: &'a SynExprResult<SynExprIdx>,
        eol_with_token_result: &'a SynExprResult<EolWithRegionalToken>,
        syn_case_branches: &'a [SynCaseBranch],
        expr_expectation: impl ExpectFluffyTerm,
    ) -> (
        SemaExprDataResult<SemaStmtData>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        let &Ok(match_target_syn_expr_idx) = match_target_syn_expr_idx_result else {
            todo!()
        };
        let (match_target_sema_expr_idx, match_target_ty) =
            self.build_sema_expr_with_ty(match_target_syn_expr_idx, ExpectAnyOriginal);
        let Some(match_target_ty) = match_target_ty else {
            use husky_print_utils::p;
            p!(self.syn_expr_region_data()[match_target_syn_expr_idx].debug(self.db()));
            p!(self.sema_expr_arena()[match_target_sema_expr_idx].debug(self.db()));
            todo!()
        };
        let mut merger = BranchTypeMerger::new(expr_expectation);
        let &Ok(eol_with_token) = eol_with_token_result else {
            todo!()
        };
        let Ok(sema_case_branches) = syn_case_branches
            .iter()
            .map(|syn_case_branch| {
                self.build_sema_case_branch(syn_case_branch, match_target_ty, &mut merger)
            })
            .collect::<SynExprResultRef<Vec<_>>>()
        else {
            todo!()
        };
        (
            Ok(SemaStmtData::Match {
                match_token,
                match_target: match_target_sema_expr_idx,
                eol_with_token,
                case_branches: sema_case_branches,
            }),
            merger
                .merge(true, self.eth_term_menu())
                .ok_or(DerivedSemaExprTypeError::BranchTypeMerge.into()),
        )
    }

    fn build_sema_case_branch<Expectation: ExpectFluffyTerm>(
        &mut self,
        syn_case_branch: &'a SynCaseBranch,
        match_target_ty: FluffyTerm,
        merger: &mut BranchTypeMerger<Expectation>,
    ) -> SynExprResultRef<'a, SemaCaseBranch> {
        let case_pattern_sema_obelisk = self.build_case_pattern_sema_obelisk(
            syn_case_branch.case_pattern_syn_obelisk.as_ref()?,
            match_target_ty,
        );
        let heavy_arrow_token = *syn_case_branch.heavy_arrow_token.as_ref()?;
        let stmts = self.build_sema_branch(*syn_case_branch.stmts.as_ref()?, merger);
        Ok(SemaCaseBranch {
            vertical_token: syn_case_branch.vertical_token,
            case_pattern_sema_obelisk,
            heavy_arrow_token,
            stmts,
        })
    }
}
