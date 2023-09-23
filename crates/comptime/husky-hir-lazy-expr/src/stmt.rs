mod branch_stmt;

use husky_syn_expr::{SynStmt, SynStmtIdx, SynStmtIdxRange};

pub use self::branch_stmt::*;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub enum HirLazyStmt {
    Let {
        pattern: HirLazyLetVariablesPattern,
        initial_value: HirLazyExprIdx,
    },
    Return {
        result: HirLazyExprIdx,
    },
    Require {
        condition: HirLazyExprIdx,
    },
    Assert {
        condition: HirLazyExprIdx,
    },
    Eval {
        expr_idx: HirLazyExprIdx,
    },
    IfElse {
        if_branch: HirLazyIfBranch,
        elif_branches: Vec<HirLazyElifBranch>,
        else_branch: Option<HirLazyElseBranch>,
    },
    Match {},
}

pub type HirLazyStmtArena = Arena<HirLazyStmt>;
pub type HirLazyStmtIdx = ArenaIdx<HirLazyStmt>;
pub type HirLazyStmtIdxRange = ArenaIdxRange<HirLazyStmt>;
pub type HirLazyStmtMap<V> = ArenaMap<HirLazyStmt, V>;

impl ToHirLazy for SynStmtIdx {
    type Output = Option<HirLazyStmt>;

    fn to_hir_lazy(&self, builder: &mut HirLazyExprBuilder) -> Self::Output {
        Some(match builder.syn_expr_region_data()[*self] {
            SynStmt::Let {
                let_token,
                ref let_variables_pattern,
                initial_value,
                ..
            } => HirLazyStmt::Let {
                pattern: builder.new_let_variables_pattern(
                    let_variables_pattern.as_ref().expect("hir stage no error"),
                ),
                initial_value: initial_value.to_hir_lazy(builder),
            },
            SynStmt::Return {
                return_token,
                result,
            } => HirLazyStmt::Return {
                result: result.to_hir_lazy(builder),
            },
            SynStmt::Require {
                require_token,
                condition,
            } => HirLazyStmt::Require {
                condition: condition.to_hir_lazy(builder),
            },
            SynStmt::Assert {
                assert_token,
                condition,
            } => HirLazyStmt::Assert {
                condition: condition.to_hir_lazy(builder),
            },
            SynStmt::Eval {
                expr_idx,
                eol_semicolon,
            } => HirLazyStmt::Eval {
                expr_idx: expr_idx.to_hir_lazy(builder),
            },
            SynStmt::Break { .. } => unreachable!(),
            SynStmt::ForBetween { .. } => unreachable!(),
            SynStmt::ForIn { .. } => unreachable!(),
            SynStmt::ForExt { .. } => unreachable!(),
            SynStmt::While { .. } => unreachable!(),
            SynStmt::DoWhile { .. } => unreachable!(),
            SynStmt::IfElse {
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
            SynStmt::Match { match_token, .. } => todo!(),
        })
    }
}

impl ToHirLazy for SynStmtIdxRange {
    type Output = HirLazyStmtIdxRange;

    fn to_hir_lazy(&self, builder: &mut HirLazyExprBuilder) -> Self::Output {
        let mut syn_stmt_indices: Vec<SynStmtIdx> = vec![];
        let mut hir_lazy_stmts: Vec<HirLazyStmt> = vec![];
        for syn_stmt_idx in self {
            match syn_stmt_idx.to_hir_lazy(builder) {
                Some(hir_lazy_stmt) => {
                    syn_stmt_indices.push(syn_stmt_idx);
                    hir_lazy_stmts.push(hir_lazy_stmt)
                }
                None => todo!(),
            }
        }
        builder.alloc_stmts(syn_stmt_indices, hir_lazy_stmts)
    }
}
