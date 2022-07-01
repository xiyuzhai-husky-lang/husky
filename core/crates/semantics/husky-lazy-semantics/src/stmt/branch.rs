use std::sync::Arc;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LazyConditionBranch {
    pub variant: LazyConditionBranchVariant,
    pub stmts: Arc<Vec<Arc<LazyStmt>>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LazyConditionBranchVariant {
    If { condition: Arc<LazyExpr> },
    Elif { condition: Arc<LazyExpr> },
    Else,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LazyPatternBranch {
    pub variant: LazyPatternBranchVariant,
    pub stmts: Arc<Vec<Arc<LazyStmt>>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LazyPatternBranchVariant {
    Case { pattern: Arc<LazyExpr> },
    Default,
}
