use crate::*;
use file::FilePtr;
use std::sync::Arc;
use text::TextRange;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcConditionBranch {
    pub variant: ProcConditionBranchVariant,
    pub stmts: Arc<Vec<Arc<ProcStmt>>>,
    pub range: TextRange,
    pub file: FilePtr,
    pub idx: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcConditionBranchVariant {
    If { condition: Arc<EagerExpr> },
    Elif { condition: Arc<EagerExpr> },
    Else,
}
