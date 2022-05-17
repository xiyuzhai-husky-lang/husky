use crate::*;
use file::FilePtr;
use std::sync::Arc;
use text::TextRange;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcBranch {
    pub variant: ProcBranchVariant,
    pub stmts: Arc<Vec<Arc<ProcStmt>>>,
    pub range: TextRange,
    pub file: FilePtr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcBranchVariant {
    If { condition: Arc<EagerExpr> },
    Elif { condition: Arc<EagerExpr> },
    Else,
    Case { pattern: Arc<EagerExpr> },
    Default,
}
