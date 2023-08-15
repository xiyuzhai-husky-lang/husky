mod branch_stmt;
mod loop_stmt;

pub use self::branch_stmt::*;
pub use self::loop_stmt::*;

use crate::*;
use husky_syn_expr::{
    ForBetweenParticulars, LetVariableDecls, LoopBoundaryKind, LoopStep, SynStmt, SynStmtIdx,
    SynStmtIdxRange,
};
use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange};

#[derive(Debug, PartialEq, Eq)]
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
    ForIn {
        condition: HirEagerExprIdx,
        block: HirEagerStmtIdxRange,
    },
    ForExt {
        expr: HirEagerExprIdx,
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

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = HirEagerExprDb)]
pub struct HirEagerForBetweenParticulars {
    pub frame_var_ident: Ident,
    pub range: HirEagerForBetweenRange,
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = HirEagerExprDb)]
pub struct HirEagerForBetweenRange {
    pub initial_boundary: HirEagerLoopBoundary,
    pub final_boundary: HirEagerLoopBoundary,
    pub step: LoopStep,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct HirEagerLoopBoundary {
    pub bound_expr: Option<HirEagerExprIdx>,
    pub kind: LoopBoundaryKind,
}

impl<'a> HirEagerExprBuilder<'a> {
    pub(crate) fn new_stmts(&mut self, syn_stmts: SynStmtIdxRange) -> HirEagerStmtIdxRange {
        let mut syn_stmt_indices: Vec<SynStmtIdx> = vec![];
        let mut hir_eager_stmts: Vec<HirEagerStmt> = vec![];
        for syn_stmt_idx in syn_stmts {
            match self.new_stmt(syn_stmt_idx) {
                Some(hir_eager_stmt) => {
                    syn_stmt_indices.push(syn_stmt_idx);
                    hir_eager_stmts.push(hir_eager_stmt)
                }
                None => todo!(),
            }
        }
        self.alloc_stmts(syn_stmt_indices, hir_eager_stmts)
    }

    pub(crate) fn new_stmt(&mut self, syn_stmt_idx: SynStmtIdx) -> Option<HirEagerStmt> {
        Some(match self.syn_expr_region_data()[syn_stmt_idx] {
            SynStmt::Let {
                let_token,
                ref let_variables_pattern,
                initial_value,
                ..
            } => HirEagerStmt::Let {
                pattern: self.new_let_variables_pattern(
                    let_variables_pattern.as_ref().expect("hir stage no error"),
                ),
                initial_value: self.new_expr(initial_value),
            },
            SynStmt::Return {
                return_token,
                result,
            } => HirEagerStmt::Return {
                result: self.new_expr(result),
            },
            SynStmt::Require {
                require_token,
                condition,
            } => HirEagerStmt::Require {
                condition: self.new_expr(condition),
            },
            SynStmt::Assert {
                assert_token,
                condition,
            } => HirEagerStmt::Assert {
                condition: self.new_expr(condition),
            },
            SynStmt::Break { break_token } => HirEagerStmt::Break,
            SynStmt::Eval {
                expr_idx,
                eol_semicolon,
            } => HirEagerStmt::Eval {
                expr_idx: self.new_expr(expr_idx),
            },
            SynStmt::ForBetween {
                for_token,
                ref particulars,
                frame_var_symbol_idx,
                ref eol_colon,
                ref block,
            } => HirEagerStmt::ForBetween {
                particulars: todo!(),
                block: self.new_stmts(*block.as_ref().expect("hir stage no errors")),
            },
            SynStmt::ForIn {
                for_token,
                ref condition,
                ref eol_colon,
                ref block,
            } => todo!(),
            SynStmt::ForExt {
                forext_token,
                expr,
                ref eol_colon,
                ref block,
            } => todo!(),
            SynStmt::While {
                while_token,
                ref condition,
                ref eol_colon,
                ref block,
            } => HirEagerStmt::While {
                condition: self.new_expr(*condition.as_ref().expect("hir stage no error")),
                stmts: self.new_stmts(*block.as_ref().expect("hir stage no error")),
            },
            SynStmt::DoWhile {
                do_token,
                while_token,
                ref condition,
                ref eol_colon,
                ref block,
            } => todo!(),
            SynStmt::IfElse {
                ref if_branch,
                ref elif_branches,
                ref else_branch,
            } => HirEagerStmt::IfElse {
                if_branch: self.new_if_branch(if_branch),
                elif_branches: elif_branches
                    .iter()
                    .map(|elif_branch| self.new_elif_branch(elif_branch))
                    .collect(),
                else_branch: else_branch
                    .as_ref()
                    .map(|else_branch| self.new_else_branch(else_branch)),
            },
            SynStmt::Match { match_token } => todo!(),
            SynStmt::Err(_) => todo!(),
        })
    }
}
