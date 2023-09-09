use husky_syn_expr::SynForextParticulars;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::debug_with_db(db = HirEagerExprDb)]
pub struct HirEagerForBetweenParticulars {
    pub frame_var_ident: Ident,
    pub range: HirEagerForBetweenRange,
}

impl ToHirEager for SynForBetweenParticulars {
    type Output = HirEagerForBetweenParticulars;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        HirEagerForBetweenParticulars {
            frame_var_ident: self.for_between_loop_var_ident,
            range: self.range.as_ref().expect("ok").to_hir_eager(builder),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::debug_with_db(db = HirEagerExprDb)]
pub struct HirEagerForBetweenRange {
    pub initial_boundary: HirEagerForBetweenLoopBoundary,
    pub final_boundary: HirEagerForBetweenLoopBoundary,
    pub step: LoopStep,
}

impl ToHirEager for SynForBetweenRange {
    type Output = HirEagerForBetweenRange;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        HirEagerForBetweenRange {
            initial_boundary: self.initial_boundary.to_hir_eager(builder),
            final_boundary: self.final_boundary.to_hir_eager(builder),
            step: self.step,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct HirEagerForBetweenLoopBoundary {
    pub bound_expr: Option<HirEagerExprIdx>,
    pub kind: LoopBoundaryKind,
}

impl ToHirEager for SynForBetweenLoopBoundary {
    type Output = HirEagerForBetweenLoopBoundary;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        HirEagerForBetweenLoopBoundary {
            bound_expr: self.bound_expr.to_hir_eager(builder),
            kind: self.kind,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct HirEagerForExtParticulars {}

impl ToHirEager for SynForextParticulars {
    type Output = HirEagerForExtParticulars;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        HirEagerForExtParticulars {}
    }
}
