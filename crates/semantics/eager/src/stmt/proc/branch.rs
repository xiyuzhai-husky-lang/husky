use std::sync::Arc;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcBranch {
    pub kind: ProcBranchKind,
    pub stmts: Arc<Vec<Arc<ProcStmt>>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcBranchKind {
    If { condition: Arc<EagerExpr> },
    Elif { condition: Arc<EagerExpr> },
    Else,
    Case { pattern: Arc<EagerExpr> },
    Default,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcBranchGroupKind {
    If,
    Switch,
    Match,
}
