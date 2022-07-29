use crate::*;
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
    Case { pattern: ProcPattern },
    Default,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcPattern {
    pub ty: EntityRoutePtr,
    pub variant: ProcPatternVariant,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcPatternVariant {
    PrimitiveLiteral(PrimitiveLiteralData),
    OneOf { subpatterns: Vec<ProcPattern> },
    EnumLiteral(EntityRoutePtr),
}
