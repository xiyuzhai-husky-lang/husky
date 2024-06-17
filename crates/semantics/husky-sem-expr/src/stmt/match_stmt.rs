use husky_regional_token::{HeavyArrowRegionalToken, VerticalRegionalToken};

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct SemCaseBranch {
    pub vertical_token: VerticalRegionalToken,
    pub case_pattern_sem_obelisk: CaseVariableObelisk,
    pub heavy_arrow_token: HeavyArrowRegionalToken,
    pub stmts: SemStmtIdxRange,
}

impl<'a> SemExprBuilder<'a> {
    pub(crate) fn build_match_stmt(
        &mut self,
        match_token: MatchRegionalToken,
        match_target_syn_expr_idx_result: &'a SynExprResult<SynExprIdx>,
        eol_with_token_result: &'a SynExprResult<EolWithRegionalToken>,
        syn_case_branches: &'a [SynCaseBranch],
        expr_expectation: impl ExpectFlyTerm,
    ) -> (SemExprDataResult<SemStmtData>, SemExprTypeResult<FlyTerm>) {
        let &Ok(match_target_syn_expr_idx) = match_target_syn_expr_idx_result else {
            todo!()
        };
        let (match_target, match_target_ty) =
            self.build_sem_expr_with_ty(match_target_syn_expr_idx, ExpectAnyOriginal);
        let Some(match_target_ty) = match_target_ty else {
            use husky_print_utils::p;
            p!(self.syn_expr_region_data()[match_target_syn_expr_idx].debug(self.db()));
            p!(self.sem_expr_arena()[match_target].debug(self.db()));
            todo!()
        };
        let mut merger = BranchTypeMerger::new(expr_expectation);
        let &Ok(eol_with_token) = eol_with_token_result else {
            todo!()
        };
        let Ok(sem_case_branches) = syn_case_branches
            .iter()
            .map(|syn_case_branch| {
                self.build_sem_case_branch(syn_case_branch, match_target_ty, &mut merger)
            })
            .collect::<SynExprResultRef<Vec<_>>>()
        else {
            todo!()
        };
        let match_contract = Contract::merge(sem_case_branches.iter().map(|branch| {
            self.syn_expr_region_data().pattern_contract(
                branch
                    .case_pattern_sem_obelisk
                    .syn_pattern_root()
                    .syn_pattern_idx(),
            )
        }));
        (
            Ok(SemStmtData::Match {
                match_token,
                match_opd: match_target,
                match_contract,
                eol_with_token,
                case_branches: sem_case_branches,
            }),
            merger
                .merge(true, self.eth_term_menu())
                .ok_or(DerivedSemExprTypeError::BranchTypeMerge.into()),
        )
    }

    fn build_sem_case_branch<Expectation: ExpectFlyTerm>(
        &mut self,
        syn_case_branch: &'a SynCaseBranch,
        match_target_ty: FlyTerm,
        merger: &mut BranchTypeMerger<Expectation>,
    ) -> SynExprResultRef<'a, SemCaseBranch> {
        let case_pattern_sem_obelisk = self.build_case_pattern_sem_obelisk(
            syn_case_branch.case_pattern_syn_obelisk.as_ref()?,
            match_target_ty,
        );
        let heavy_arrow_token = *syn_case_branch.heavy_arrow_token.as_ref()?;
        let stmts = self.build_sem_branch(*syn_case_branch.stmts.as_ref()?, merger);
        Ok(SemCaseBranch {
            vertical_token: syn_case_branch.vertical_token,
            case_pattern_sem_obelisk,
            heavy_arrow_token,
            stmts,
        })
    }
}
