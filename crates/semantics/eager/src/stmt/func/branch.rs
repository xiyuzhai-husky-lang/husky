use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncConditionBranch {
    pub kind: DeclBranchKind,
    pub stmts: Arc<Vec<Arc<FuncStmt>>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeclBranchKind {
    If { condition: Arc<EagerExpr> },
    Elif { condition: Arc<EagerExpr> },
    Else,
    Case { pattern: Arc<EagerExpr> },
    Default,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclBranchGroupKind {
    If,
    Switch,
    Match,
}
