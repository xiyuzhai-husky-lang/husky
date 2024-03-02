use crate::{engine::PlaceContractEngine, site::SemaPlaceContractSite};
use husky_sema_expr::{SemaStmtData, SemaStmtIdx, SemaStmtIdxRange};
use husky_term_prelude::TermContract;

impl<'a> PlaceContractEngine<'a> {
    pub(crate) fn infer_stmts(
        &mut self,
        stmts: SemaStmtIdxRange,
        contract: TermContract,
        site: SemaPlaceContractSite,
    ) {
        let (non_last_stmts, last_stmt) = stmts.split_last();
        for non_last_stmt in non_last_stmts {
            self.infer_stmt(non_last_stmt, TermContract::Pure, Default::default())
        }
        self.infer_stmt(last_stmt, contract, site)
    }

    fn infer_stmt(
        &mut self,
        stmt: SemaStmtIdx,
        contract: TermContract,
        site: SemaPlaceContractSite,
    ) {
        match *stmt.data(self.sema_expr_region_data().sema_stmt_arena()) {
            SemaStmtData::Let {
                let_token,
                let_pattern_sema_obelisk,
                contract,
                eq_token,
                initial_value_sema_expr_idx,
                ref coersion_outcome,
            } => todo!(),
            SemaStmtData::Return {
                result,
                ref coersion_outcome,
                ..
            } => todo!(),
            SemaStmtData::Require { condition, .. } => todo!(),
            SemaStmtData::Assert { condition, .. } => todo!(),
            SemaStmtData::Break { .. } => todo!(),
            SemaStmtData::Eval {
                sema_expr_idx,
                ref outcome,
                eol_semicolon,
                ..
            } => {
                let discarded = eol_semicolon.is_some();
                let expr_contract = if discarded {
                    TermContract::Pure
                } else {
                    contract
                };
                self.infer_expr(sema_expr_idx, expr_contract, site)
            }
            SemaStmtData::ForBetween {
                ref particulars,
                for_loop_var_symbol_idx,
                eol_colon,
                block,
                ..
            } => todo!(),
            SemaStmtData::ForIn {
                condition, block, ..
            } => todo!(),
            SemaStmtData::Forext {
                ref particulars,
                block,
                ..
            } => todo!(),
            SemaStmtData::While {
                condition, block, ..
            } => todo!(),
            SemaStmtData::DoWhile {
                condition, block, ..
            } => todo!(),
            SemaStmtData::IfElse {
                ref if_branch,
                ref elif_branches,
                ref else_branch,
            } => todo!(),
            SemaStmtData::Match {
                match_target,
                ref case_branches,
                ..
            } => todo!(),
        }
    }
}
