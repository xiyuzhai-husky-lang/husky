use crate::*;
use husky_ast::CasePattern;
use husky_file::FilePtr;
use husky_text::TextRange;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcPatternBranch {
    pub variant: ProcPatternBranchVariant,
    pub stmts: Arc<Vec<Arc<ProcStmt>>>,
    pub range: TextRange,
    pub file: FilePtr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcPatternBranchVariant {
    Case { pattern: CasePattern },
    Default,
}

impl ProcPatternBranch {
    pub(crate) fn needs_context(&self) -> bool {
        self.stmts.iter().any(|stmt| stmt.needs_context)
    }
}
