mod branch_stmt;

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
