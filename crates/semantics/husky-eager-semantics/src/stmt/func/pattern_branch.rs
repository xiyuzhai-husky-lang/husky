use crate::*;
use husky_entity_path::EntityPathItd;
use husky_primitive_literal_syntax::RawLiteralData;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncStmtPatternBranch {
    pub variant: FuncStmtPatternBranchVariant,
    pub stmts: Arc<Vec<Arc<FuncStmt>>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FuncStmtPatternBranchVariant {
    Case { pattern: FuncStmtPattern },
    Default,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncStmtPattern {
    pub ty: Ty,
    pub variant: FuncStmtPatternVariant,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FuncStmtPatternVariant {
    PrimitiveLiteral(RawLiteralData),
    OneOf { subpatterns: Vec<FuncStmtPattern> },
    EnumLiteral(EntityPathItd),
}
