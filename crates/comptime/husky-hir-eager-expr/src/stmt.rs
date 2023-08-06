mod branch_stmt;
mod loop_stmt;

pub use self::branch_stmt::*;
pub use self::loop_stmt::*;

use crate::*;
use husky_syn_expr::{ForBetweenParticulars, LetVariableDecls, LoopBoundaryKind, LoopStep};
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
        particulars: HirForBetweenParticulars,
        frame_var_symbol_idx: CurrentHirEagerSymbolIdx,
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
        block: HirEagerStmtIdxRange,
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
pub struct HirForBetweenParticulars {
    pub frame_var_ident: Ident,
    pub range: HirForBetweenRange,
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = HirEagerExprDb)]
pub struct HirForBetweenRange {
    pub initial_boundary: HirLoopBoundary,
    pub final_boundary: HirLoopBoundary,
    pub step: LoopStep,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct HirLoopBoundary {
    pub bound_expr: Option<HirEagerExprIdx>,
    pub kind: LoopBoundaryKind,
}
