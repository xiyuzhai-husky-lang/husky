use super::{expr::VmirExprIdx, ToVmir, VmirBuilder};
use husky_expr::stmt::{LoopBoundaryKind, LoopStep};
use husky_hir_eager_expr::{
    HirEagerForBetweenLoopBoundary, HirEagerForBetweenParticulars, HirEagerForBetweenRange,
};
use husky_linket_impl::linket_impl::IsLinketImpl;
use husky_place::place::idx::PlaceIdx;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct VmirForBetweenParticulars<LinketImpl: IsLinketImpl> {
    for_loop_variable_place_idx: PlaceIdx,
    range: VmirForBetweenRange<LinketImpl>,
}

impl<LinketImpl: IsLinketImpl> VmirForBetweenParticulars<LinketImpl> {
    pub fn for_loop_variable_place_idx(&self) -> PlaceIdx {
        self.for_loop_variable_place_idx
    }

    pub fn range(&self) -> &VmirForBetweenRange<LinketImpl> {
        &self.range
    }
}

impl<LinketImpl: IsLinketImpl> ToVmir<LinketImpl> for &HirEagerForBetweenParticulars {
    type Output = VmirForBetweenParticulars<LinketImpl>;

    fn to_vmir<Linktime>(self, builder: &mut VmirBuilder<Linktime>) -> Self::Output
    where
        Linktime: husky_linktime::IsLinktime<LinketImpl = LinketImpl>,
    {
        VmirForBetweenParticulars {
            for_loop_variable_place_idx: self.frame_var_place_idx,
            range: self.range.to_vmir(builder),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db]
pub struct VmirForBetweenRange<LinketImpl: IsLinketImpl> {
    pub initial_boundary: VmirForBetweenLoopBoundary<LinketImpl>,
    pub final_boundary: VmirForBetweenLoopBoundary<LinketImpl>,
    pub step: LoopStep,
}

impl<LinketImpl: IsLinketImpl> ToVmir<LinketImpl> for &HirEagerForBetweenRange {
    type Output = VmirForBetweenRange<LinketImpl>;

    fn to_vmir<Linktime>(self, builder: &mut VmirBuilder<Linktime>) -> Self::Output
    where
        Linktime: husky_linktime::IsLinktime<LinketImpl = LinketImpl>,
    {
        VmirForBetweenRange {
            initial_boundary: self.initial_boundary.to_vmir(builder),
            final_boundary: self.final_boundary.to_vmir(builder),
            step: self.step,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct VmirForBetweenLoopBoundary<LinketImpl: IsLinketImpl> {
    pub bound_expr: Option<VmirExprIdx<LinketImpl>>,
    pub kind: LoopBoundaryKind,
}

impl<LinketImpl: IsLinketImpl> ToVmir<LinketImpl> for &HirEagerForBetweenLoopBoundary {
    type Output = VmirForBetweenLoopBoundary<LinketImpl>;

    fn to_vmir<Linktime>(self, builder: &mut VmirBuilder<Linktime>) -> Self::Output
    where
        Linktime: husky_linktime::IsLinktime<LinketImpl = LinketImpl>,
    {
        VmirForBetweenLoopBoundary {
            bound_expr: self.bound_expr.as_ref().map(|expr| expr.to_vmir(builder)),
            kind: self.kind,
        }
    }
}
