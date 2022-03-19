use std::sync::Arc;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImprBranch {
    pub kind: ImprBranchKind,
    pub stmts: Arc<Vec<Arc<ImprStmt>>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImprBranchKind {
    If { condition: Arc<EagerExpr> },
    Elif { condition: Arc<EagerExpr> },
    Else,
    Case { pattern: Arc<EagerExpr> },
    Default,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImprBranchGroupKind {
    If,
    Switch,
    Match,
}
