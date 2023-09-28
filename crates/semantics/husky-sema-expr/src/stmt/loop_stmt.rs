use husky_coword::Ident;
use husky_opr::BinaryComparisonOpr;
use husky_regional_token::RegionalTokenIdx;

use super::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SemaExprDb)]
pub struct SemaForBetweenParticulars {
    for_between_loop_var_regional_token_idx: RegionalTokenIdx,
    for_between_loop_var_ident: Ident,
    for_between_loop_var_expr_idx: SemaExprIdx,
    range: SemaForBetweenRange,
}

impl SemaForBetweenParticulars {
    pub fn for_between_loop_var_regional_token_idx(&self) -> RegionalTokenIdx {
        self.for_between_loop_var_regional_token_idx
    }

    pub fn for_between_loop_var_ident(&self) -> Ident {
        self.for_between_loop_var_ident
    }

    pub fn for_between_loop_var_expr_idx(&self) -> SemaExprIdx {
        self.for_between_loop_var_expr_idx
    }

    pub fn range(&self) -> &SemaForBetweenRange {
        &self.range
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SemaExprDb)]
pub struct SemaForBetweenRange {
    pub initial_boundary: SemaForBetweenLoopBoundary,
    pub final_boundary: SemaForBetweenLoopBoundary,
    pub step: LoopStep,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SemaForBetweenLoopBoundary {
    pub bound_expr: Option<SemaExprIdx>,
    pub kind: LoopBoundaryKind,
}

impl Default for SemaForBetweenLoopBoundary {
    fn default() -> Self {
        Self {
            bound_expr: None,
            kind: LoopBoundaryKind::LowerClosed,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SemaForextParticulars {
    pub forext_loop_var_regional_token_idx: RegionalTokenIdx,
    pub forext_loop_var_ident: Ident,
    pub forext_loop_var_expr_idx: SemaExprIdx,
    pub bound_expr: SemaExprIdx,
    pub boundary_kind: LoopBoundaryKind,
}

impl SemaForextParticulars {
    pub(crate) fn new(
        forext_loop_var_regional_token_idx: RegionalTokenIdx,
        forext_loop_var_ident: Ident,
        forext_loop_var_expr_idx: SemaExprIdx,
        opr: BinaryComparisonOpr,
        bound_expr: SemaExprIdx,
    ) -> Self {
        todo!()
    }
}
