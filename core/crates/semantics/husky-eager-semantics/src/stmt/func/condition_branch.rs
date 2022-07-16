use husky_file::FilePtr;
use husky_text::TextRange;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncConditionFlowBranch {
    pub variant: FuncConditionFlowBranchVariant,
    pub stmts: Arc<Vec<Arc<FuncStmt>>>,
    pub range: TextRange,
    pub file: FilePtr,
    pub idx: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FuncConditionFlowBranchVariant {
    If { condition: Arc<EagerExpr> },
    Elif { condition: Arc<EagerExpr> },
    Else,
}

impl FuncConditionFlowBranch {
    pub fn needs_context(&self) -> bool {
        self.variant.needs_context() || self.stmts.iter().any(|stmt| stmt.needs_context())
    }
}

impl FuncConditionFlowBranchVariant {
    pub fn needs_context(&self) -> bool {
        match self {
            FuncConditionFlowBranchVariant::If { ref condition } => condition.needs_context,
            FuncConditionFlowBranchVariant::Elif { ref condition } => condition.needs_context,
            FuncConditionFlowBranchVariant::Else => false,
        }
    }
}
