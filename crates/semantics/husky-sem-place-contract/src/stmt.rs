use crate::{engine::PlaceContractEngine, site::SemPlaceContractSite};
use husky_sem_expr::{stmt::condition::SemCondition, SemStmtData, SemStmtIdx, SemStmtIdxRange};
use husky_term_prelude::Contract;

impl<'a> PlaceContractEngine<'a> {
    pub(crate) fn infer_stmts(
        &mut self,
        stmts: SemStmtIdxRange,
        contract: Contract,
        site: SemPlaceContractSite,
    ) {
        let (non_last_stmts, last_stmt) = stmts.split_last();
        for non_last_stmt in non_last_stmts {
            self.infer_stmt(non_last_stmt, Contract::Pure, Default::default())
        }
        self.infer_stmt(last_stmt, contract, site)
    }

    fn infer_stmt(&mut self, stmt: SemStmtIdx, contract: Contract, site: SemPlaceContractSite) {
        match *stmt.data(self.sem_expr_region_data().sem_stmt_arena()) {
            SemStmtData::Let {
                contract,
                initial_value: initial_value_sem_expr_idx,
                ..
            } => self.infer_expr(initial_value_sem_expr_idx, contract, Default::default()),
            SemStmtData::Return { result, .. } => {
                self.infer_expr(result, Contract::Move, Default::default())
            }
            SemStmtData::Require { condition, .. } | SemStmtData::Assert { condition, .. } => {
                self.infer_condition(condition);
            }
            SemStmtData::Break { .. } => (),
            SemStmtData::Eval {
                expr: sem_expr_idx,
                eol_semicolon,
                ..
            } => {
                let discarded = eol_semicolon.is_some();
                let expr_contract = if discarded { Contract::Pure } else { contract };
                self.infer_expr(sem_expr_idx, expr_contract, site)
            }
            SemStmtData::ForBetween {
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
            SemStmtData::ForIn { .. } => {
                todo!()
            }
            SemStmtData::Forext {
                ref particulars,
                stmts,
                ..
            } => {
                self.infer_expr(
                    particulars.bound_expr,
                    Contract::Pure,
                    Default::default(),
                );
                self.infer_stmts(stmts, Contract::Pure, Default::default());
            }
            SemStmtData::While {
                condition, stmts, ..
            }
            | SemStmtData::DoWhile {
                condition, stmts, ..
            } => {
                self.infer_condition(condition);
                self.infer_stmts(stmts, Contract::Pure, Default::default());
            }
            SemStmtData::IfElse {
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
            SemStmtData::Match {
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
            // ad hoc
            SemStmtData::Narrate { narrate_token } => (),
        }
    }

    fn infer_condition(&mut self, condition: SemCondition) {
        let expr = match condition {
            SemCondition::Be { src, .. } => src,
            SemCondition::Other { sem_expr_idx, .. } => sem_expr_idx,
        };
        self.infer_expr(expr, Contract::Pure, Default::default())
    }
}
