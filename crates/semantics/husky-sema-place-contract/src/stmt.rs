use crate::{engine::PlaceContractEngine, site::SemaPlaceContractSite};
use husky_sema_expr::{
    stmt::condition::SemaCondition, SemaStmtData, SemaStmtIdx, SemaStmtIdxRange,
};
use husky_term_prelude::Contract;

impl<'a> PlaceContractEngine<'a> {
    pub(crate) fn infer_stmts(
        &mut self,
        stmts: SemaStmtIdxRange,
        contract: Contract,
        site: SemaPlaceContractSite,
    ) {
        let (non_last_stmts, last_stmt) = stmts.split_last();
        for non_last_stmt in non_last_stmts {
            self.infer_stmt(non_last_stmt, Contract::Pure, Default::default())
        }
        self.infer_stmt(last_stmt, contract, site)
    }

    fn infer_stmt(&mut self, stmt: SemaStmtIdx, contract: Contract, site: SemaPlaceContractSite) {
        match *stmt.data(self.sema_expr_region_data().sema_stmt_arena()) {
            SemaStmtData::Let {
                contract,
                initial_value_sema_expr_idx,
                ..
            } => self.infer_expr(initial_value_sema_expr_idx, contract, Default::default()),
            SemaStmtData::Return { result, .. } => {
                self.infer_expr(result, Contract::Move, Default::default())
            }
            SemaStmtData::Require { condition, .. } | SemaStmtData::Assert { condition, .. } => {
                self.infer_condition(condition);
            }
            SemaStmtData::Break { .. } => (),
            SemaStmtData::Eval {
                sema_expr_idx,
                eol_semicolon,
                ..
            } => {
                let discarded = eol_semicolon.is_some();
                let expr_contract = if discarded { Contract::Pure } else { contract };
                self.infer_expr(sema_expr_idx, expr_contract, site)
            }
            SemaStmtData::ForBetween {
                ref particulars,

                stmts,
                ..
            } => {
                particulars
                    .range()
                    .initial_boundary
                    .bound_expr
                    .map(|expr| self.infer_expr(expr, Contract::Pure, Default::default()));
                particulars
                    .range()
                    .final_boundary
                    .bound_expr
                    .map(|expr| self.infer_expr(expr, Contract::Pure, Default::default()));
                self.infer_stmts(stmts, Contract::Pure, Default::default());
            }
            SemaStmtData::ForIn { .. } => {
                todo!()
            }
            SemaStmtData::Forext {
                ref particulars,
                stmts,
                ..
            } => {
                self.infer_expr(
                    particulars.bound_expr_sema_expr_idx,
                    Contract::Pure,
                    Default::default(),
                );
                self.infer_stmts(stmts, Contract::Pure, Default::default());
            }
            SemaStmtData::While {
                condition, stmts, ..
            }
            | SemaStmtData::DoWhile {
                condition, stmts, ..
            } => {
                self.infer_condition(condition);
                self.infer_stmts(stmts, Contract::Pure, Default::default());
            }
            SemaStmtData::IfElse {
                ref if_branch,
                ref elif_branches,
                ref else_branch,
            } => {
                self.infer_condition(if_branch.condition);
                self.infer_stmts(if_branch.stmts, contract, site.clone());
                for elif_branch in elif_branches {
                    self.infer_condition(elif_branch.condition);
                    self.infer_stmts(elif_branch.stmts, contract, site.clone());
                }
                if let Some(else_branch) = else_branch {
                    self.infer_stmts(else_branch.stmts, contract, site.clone());
                }
            }
            SemaStmtData::Match {
                match_opd: match_target,
                ref case_branches,
                ..
            } => {
                // todo: match_contract
                let match_contract = Contract::Pure;
                self.infer_expr(match_target, match_contract, Default::default());
                for case_branch in case_branches {
                    self.infer_stmts(case_branch.stmts, contract, site.clone());
                }
            }
        }
    }

    fn infer_condition(&mut self, condition: SemaCondition) {
        let expr = match condition {
            SemaCondition::Be { src, .. } => src,
            SemaCondition::Other { sema_expr_idx, .. } => sema_expr_idx,
        };
        self.infer_expr(expr, Contract::Pure, Default::default())
    }
}
