use crate::*;
use husky_primitive_literal_syntax::PrimitiveLiteralData;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncPatternBranch {
    pub variant: FuncPatternBranchVariant,
    pub stmts: Arc<Vec<Arc<FuncStmt>>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FuncPatternBranchVariant {
    Case { pattern: FuncPattern },
    Default,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncPattern {
    pub ty: EntityRoutePtr,
    pub variant: FuncPatternVariant,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FuncPatternVariant {
    PrimitiveLiteral(PrimitiveLiteralData),
    OneOf { subpatterns: Vec<FuncPattern> },
    EnumLiteral(EntityRoutePtr),
}
