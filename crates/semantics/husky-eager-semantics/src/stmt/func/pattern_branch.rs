use crate::*;
use husky_primitive_literal_syntax::PrimitiveLiteralData;

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
    pub ty: EntityRoutePtr,
    pub variant: FuncStmtPatternVariant,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FuncStmtPatternVariant {
    PrimitiveLiteral(PrimitiveLiteralData),
    OneOf { subpatterns: Vec<FuncStmtPattern> },
    EnumLiteral(EntityRoutePtr),
}
