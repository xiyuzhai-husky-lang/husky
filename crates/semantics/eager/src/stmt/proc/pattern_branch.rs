use crate::*;
use file::FilePtr;
use std::sync::Arc;
use text::TextRange;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcPatternBranch {
    pub variant: ProcConditionBranchVariant,
    pub stmts: Arc<Vec<Arc<ProcStmt>>>,
    pub range: TextRange,
    pub file: FilePtr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcPatternBranchVariant {
    Case { pattern: Arc<EagerExpr> },
    Default,
}
