use crate::*;
use husky_ast::RawCasePattern;
use husky_file::FilePtr;
use husky_primitive_literal_syntax::PrimitiveLiteralData;
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
    Case { pattern: ProcCasePattern },
    Default,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcCasePattern {
    pub ty: EntityRoutePtr,
    pub variant: ProcCasePatternVariant,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcCasePatternVariant {
    PrimitiveLiteral(PrimitiveLiteralData),
    OneOf { subpatterns: Vec<ProcCasePattern> },
    EnumLiteral(EntityRoutePtr),
}
