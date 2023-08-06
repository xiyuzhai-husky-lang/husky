use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct HirEagerLetVariablesPattern {
    pattern_expr_idx: HirEagerPatternExprIdx,
    variables: CurrentHirEagerSymbolIdxRange,
    ty: Option<HirEagerExprIdx>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct HirEagerBeVariablesPattern {}
