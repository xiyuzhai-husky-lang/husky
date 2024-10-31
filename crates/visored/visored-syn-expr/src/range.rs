use crate::expr::VdSynExprMap;
use latex_ast::ast::{LxAstIdx, LxAstIdxRange};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdSynExprRange {
    Ast(LxAstIdx),
    Asts(LxAstIdxRange),
}

pub type VdSynExprRangeMap<'a> = VdSynExprMap<VdSynExprRange>;
