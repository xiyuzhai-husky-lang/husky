use crate::*;
use husky_file::FilePtr;
use husky_text::TextRange;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcConditionFlowBranch {
    pub variant: ProcConditionFlowBranchVariant,
    pub stmts: Arc<Vec<Arc<ProcStmt>>>,
    pub range: TextRange,
    pub file: FilePtr,
    pub idx: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcConditionFlowBranchVariant {
    If { condition: Arc<EagerExpr> },
    Elif { condition: Arc<EagerExpr> },
    Else,
}

impl ProcConditionFlowBranch {
    pub fn needs_context(&self) -> bool {
        self.variant.needs_context() || self.stmts.iter().any(|stmt| stmt.needs_context)
    }
}

impl ProcConditionFlowBranchVariant {
    pub fn needs_context(&self) -> bool {
        match self {
            ProcConditionFlowBranchVariant::If { ref condition } => condition.needs_context,
            ProcConditionFlowBranchVariant::Elif { ref condition } => condition.needs_context,
            ProcConditionFlowBranchVariant::Else => false,
        }
    }
}
