use file::FilePtr;
use text::TextRange;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncConditionBranch {
    pub variant: FuncConditionBranchVariant,
    pub stmts: Arc<Vec<Arc<FuncStmt>>>,
    pub range: TextRange,
    pub file: FilePtr,
    pub idx: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FuncConditionBranchVariant {
    If { condition: Arc<EagerExpr> },
    Elif { condition: Arc<EagerExpr> },
    Else,
}
