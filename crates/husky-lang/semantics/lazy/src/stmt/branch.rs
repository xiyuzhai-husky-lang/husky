use std::sync::Arc;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LazyBranch {
    pub kind: LazyBranchKind,
    pub stmts: Arc<Vec<Arc<LazyStmt>>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LazyBranchKind {
    If { condition: Arc<LazyExpr> },
    Elif { condition: Arc<LazyExpr> },
    Else,
    Case { pattern: Arc<LazyExpr> },
    Default,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LazyBranchGroupKind {
    If,
    Switch,
    Match,
}
