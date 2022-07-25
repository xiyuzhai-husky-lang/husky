use crate::*;
use husky_ast::RawCasePattern;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncPatternBranch {
    pub variant: FuncPatternBranchVariant,
    pub stmts: Arc<Vec<Arc<FuncStmt>>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FuncPatternBranchVariant {
    Case { pattern: RawCasePattern },
    Default,
}
