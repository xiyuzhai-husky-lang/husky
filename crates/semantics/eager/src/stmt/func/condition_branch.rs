use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncConditionBranch {
    pub kind: FuncConditionBranchKind,
    pub stmts: Arc<Vec<Arc<FuncStmt>>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FuncConditionBranchKind {
    If { condition: Arc<EagerExpr> },
    Elif { condition: Arc<EagerExpr> },
    Else,
}
