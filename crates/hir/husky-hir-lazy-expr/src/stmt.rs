mod branch_stmt;

use husky_sema_expr::{SemaCondition, SemaStmtData, SemaStmtIdx, SemaStmtIdxRange};

use idx_arena::ArenaRef;

pub use self::branch_stmt::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::debug_with_db(db = HirLazyExprDb, jar = HirLazyExprJar)]
pub enum HirLazyStmt {
    Let {
        pattern: HirLazyLetVariablesPattern,
        initial_value: HirLazyExprIdx,
    },
    Return {
        result: HirLazyExprIdx,
    },
    Require {
        condition: HirLazyCondition,
    },
    Assert {
        condition: HirLazyCondition,
    },
    Eval {
        expr_idx: HirLazyExprIdx,
        discarded: bool,
    },
    IfElse {
        if_branch: HirLazyIfBranch,
        elif_branches: Vec<HirLazyElifBranch>,
        else_branch: Option<HirLazyElseBranch>,
    },
    Match {},
}

pub type HirLazyStmtArena = Arena<HirLazyStmt>;
pub type HirLazyStmtArenaRef<'a> = ArenaRef<'a, HirLazyStmt>;
pub type HirLazyStmtIdx = ArenaIdx<HirLazyStmt>;
pub type HirLazyStmtIdxRange = ArenaIdxRange<HirLazyStmt>;
pub type HirLazyStmtMap<V> = ArenaMap<HirLazyStmt, V>;

impl ToHirLazy for SemaStmtIdx {
    type Output = Option<HirLazyStmt>;

    fn to_hir_lazy(&self, builder: &mut HirLazyExprBuilder) -> Self::Output {
        Some(match self.data(builder.sema_stmt_arena_ref()) {
            SemaStmtData::Let {
                let_token: _,
                let_pattern_sema_obelisk: let_variables_pattern,
                initial_value_sema_expr_idx: initial_value,
                ..
            } => HirLazyStmt::Let {
                pattern: builder.new_let_variables_pattern(let_variables_pattern),
                initial_value: initial_value.to_hir_lazy(builder),
            },
            SemaStmtData::Return {
                return_token: _,
                result,
            } => HirLazyStmt::Return {
                result: result.to_hir_lazy(builder),
            },
            SemaStmtData::Require {
                require_token: _,
                condition,
            } => HirLazyStmt::Require {
                condition: condition.to_hir_lazy(builder),
            },
            SemaStmtData::Assert {
                assert_token: _,
                condition,
            } => HirLazyStmt::Assert {
                condition: condition.to_hir_lazy(builder),
            },
            SemaStmtData::Eval {
                sema_expr_idx: expr_idx,
                outcome,
                eol_semicolon,
            } => HirLazyStmt::Eval {
                expr_idx: expr_idx.to_hir_lazy(builder),
                discarded: eol_semicolon.as_ref().expect("no error").is_some(),
            },
            SemaStmtData::Break { .. } => unreachable!(),
            SemaStmtData::ForBetween { .. } => unreachable!(),
            SemaStmtData::ForIn { .. } => unreachable!(),
            SemaStmtData::Forext { .. } => unreachable!(),
            SemaStmtData::While { .. } => unreachable!(),
            SemaStmtData::DoWhile { .. } => unreachable!(),
            SemaStmtData::IfElse {
                ref if_branch,
                ref elif_branches,
                ref else_branch,
            } => HirLazyStmt::IfElse {
                if_branch: if_branch.to_hir_lazy(builder),
                elif_branches: elif_branches
                    .iter()
                    .map(|elif_branch| elif_branch.to_hir_lazy(builder))
                    .collect(),
                else_branch: else_branch
                    .as_ref()
                    .map(|else_branch| else_branch.to_hir_lazy(builder)),
            },
            SemaStmtData::Match { .. } => todo!(),
        })
    }
}

impl ToHirLazy for SemaStmtIdxRange {
    type Output = HirLazyStmtIdxRange;

    fn to_hir_lazy(&self, builder: &mut HirLazyExprBuilder) -> Self::Output {
        let mut sema_stmt_indices: Vec<SemaStmtIdx> = vec![];
        let mut hir_lazy_stmts: Vec<HirLazyStmt> = vec![];
        for sema_stmt_idx in self {
            match sema_stmt_idx.to_hir_lazy(builder) {
                Some(hir_lazy_stmt) => {
                    sema_stmt_indices.push(sema_stmt_idx);
                    hir_lazy_stmts.push(hir_lazy_stmt)
                }
                None => todo!(),
            }
        }
        builder.alloc_stmts(sema_stmt_indices, hir_lazy_stmts)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HirLazyCondition {
    Be {
        src: HirLazyExprIdx,
        target: HirLazyBeVariablesPattern,
    },
    Other(HirLazyExprIdx),
}

impl ToHirLazy for SemaCondition {
    // ad hoc
    type Output = HirLazyCondition;

    fn to_hir_lazy(&self, builder: &mut HirLazyExprBuilder) -> Self::Output {
        match *self {
            SemaCondition::Be {
                src,
                be_regional_token_idx,
                target,
            } => HirLazyCondition::Be {
                src: src.to_hir_lazy(builder),
                target: target.to_hir_lazy(builder),
            },
            SemaCondition::Other(sema_expr_idx) => {
                HirLazyCondition::Other(sema_expr_idx.to_hir_lazy(builder))
            }
        }
    }
}
