use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImprBranch {
    pub kind: ImprBranchKind,
    pub stmts: Arc<Vec<Arc<ImprStmt>>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImprBranchKind {
    If { condition: Arc<Expr> },
    Elif { condition: Arc<Expr> },
    Else,
    Case { pattern: Arc<Expr> },
    Default,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImprBranchGroupKind {
    If,
    Switch,
    Match,
}
