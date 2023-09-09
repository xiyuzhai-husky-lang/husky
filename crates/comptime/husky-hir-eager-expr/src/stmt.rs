mod branch_stmt;
mod loop_stmt;

pub use self::branch_stmt::*;
pub use self::loop_stmt::*;

use crate::*;
use husky_syn_expr::{
    LetVariableObelisk, LoopBoundaryKind, LoopStep, SynForBetweenLoopBoundary,
    SynForBetweenParticulars, SynForBetweenRange, SynStmt, SynStmtIdx, SynStmtIdxRange,
};
use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange};
use salsa::debug::ExpectWithDb;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum HirEagerStmt {
    Let {
        pattern: HirEagerLetVariablesPattern,
        initial_value: HirEagerExprIdx,
    },
    Return {
        result: HirEagerExprIdx,
    },
    Require {
        condition: HirEagerExprIdx,
    },
    Assert {
        condition: HirEagerExprIdx,
    },
    Break,
    Eval {
        expr_idx: HirEagerExprIdx,
    },
    ForBetween {
        particulars: HirEagerForBetweenParticulars,
        // frame_var_symbol_idx: CurrentHirEagerSymbolIdx,
        block: HirEagerStmtIdxRange,
    },
    ForExt {
        particulars: HirEagerForExtParticulars,
        block: HirEagerStmtIdxRange,
    },
    ForIn {
        condition: HirEagerExprIdx,
        block: HirEagerStmtIdxRange,
    },
    While {
        condition: HirEagerExprIdx,
        stmts: HirEagerStmtIdxRange,
    },
    DoWhile {
        condition: HirEagerExprIdx,
        block: HirEagerStmtIdxRange,
    },
    IfElse {
        if_branch: HirEagerIfBranch,
        elif_branches: Vec<HirEagerElifBranch>,
        else_branch: Option<HirEagerElseBranch>,
    },
    Match {},
}

pub type HirEagerStmtArena = Arena<HirEagerStmt>;
pub type HirEagerStmtIdx = ArenaIdx<HirEagerStmt>;
pub type HirEagerStmtIdxRange = ArenaIdxRange<HirEagerStmt>;
pub type HirEagerStmtMap<V> = ArenaMap<HirEagerStmt, V>;

impl ToHirEager for SynStmtIdx {
    type Output = Option<HirEagerStmt>;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        Some(match builder.syn_expr_region_data()[*self] {
            SynStmt::Let {
                let_token,
                ref let_variables_pattern,
                initial_value,
                ..
            } => HirEagerStmt::Let {
                pattern: builder.new_let_variables_pattern(
                    let_variables_pattern.as_ref().expect("hir stage no error"),
                ),
                initial_value: initial_value.to_hir_eager(builder),
            },
            SynStmt::Return {
                return_token,
                result,
            } => HirEagerStmt::Return {
                result: result.to_hir_eager(builder),
            },
            SynStmt::Require {
                require_token,
                condition,
            } => HirEagerStmt::Require {
                condition: condition.to_hir_eager(builder),
            },
            SynStmt::Assert {
                assert_token,
                condition,
            } => HirEagerStmt::Assert {
                condition: condition.to_hir_eager(builder),
            },
            SynStmt::Break { break_token } => HirEagerStmt::Break,
            SynStmt::Eval {
                expr_idx,
                eol_semicolon,
            } => HirEagerStmt::Eval {
                expr_idx: expr_idx.to_hir_eager(builder),
            },
            SynStmt::ForBetween {
                for_token,
                ref particulars,
                frame_var_symbol_idx,
                ref eol_colon,
                ref block,
            } => HirEagerStmt::ForBetween {
                particulars: particulars.to_hir_eager(builder),
                block: block
                    .as_ref()
                    .expect("hir stage no errors")
                    .to_hir_eager(builder),
            },
            SynStmt::ForIn {
                for_token,
                ref condition,
                ref eol_colon,
                ref block,
            } => todo!(),
            SynStmt::ForExt {
                forext_token,
                ref particulars,
                ref eol_colon,
                ref block,
            } => HirEagerStmt::ForExt {
                particulars: particulars.to_hir_eager(builder),
                block: block
                    .as_ref()
                    .expect("hir stage no error")
                    .to_hir_eager(builder),
            },
            SynStmt::While {
                ref condition,
                ref block,
                ..
            } => HirEagerStmt::While {
                condition: condition
                    .as_ref()
                    .expect("hir stage no error")
                    .to_hir_eager(builder),
                stmts: block
                    .as_ref()
                    .expect("hir stage no error")
                    .to_hir_eager(builder),
            },
            SynStmt::DoWhile {
                ref condition,
                ref block,
                ..
            } => HirEagerStmt::DoWhile {
                condition: condition
                    .as_ref()
                    .expect("hir stage no errors")
                    .to_hir_eager(builder),
                block: block
                    .as_ref()
                    .expect("hir stage no errors")
                    .to_hir_eager(builder),
            },
            SynStmt::IfElse {
                ref if_branch,
                ref elif_branches,
                ref else_branch,
            } => HirEagerStmt::IfElse {
                if_branch: if_branch.to_hir_eager(builder),
                elif_branches: elif_branches
                    .iter()
                    .map(|elif_branch| elif_branch.to_hir_eager(builder))
                    .collect(),
                else_branch: else_branch
                    .as_ref()
                    .map(|else_branch| else_branch.to_hir_eager(builder)),
            },
            SynStmt::Match { match_token } => HirEagerStmt::Match {},
        })
    }
}

impl ToHirEager for SynStmtIdxRange {
    type Output = HirEagerStmtIdxRange;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        let mut syn_stmt_indices: Vec<SynStmtIdx> = vec![];
        let mut hir_eager_stmts: Vec<HirEagerStmt> = vec![];
        for syn_stmt_idx in self {
            match syn_stmt_idx.to_hir_eager(builder) {
                Some(hir_eager_stmt) => {
                    syn_stmt_indices.push(syn_stmt_idx);
                    hir_eager_stmts.push(hir_eager_stmt)
                }
                None => todo!(),
            }
        }
        builder.alloc_stmts(syn_stmt_indices, hir_eager_stmts)
    }
}
