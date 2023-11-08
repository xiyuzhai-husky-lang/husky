pub mod control_flow;

use crate::{source_map::HirLazyExprSourceMap, *};
use husky_sema_expr::SemaExprIdx;
use husky_syn_expr::{SynExprIdx, SynExprRegion};

#[inline(always)]
pub fn hir_lazy_body_with_expr_region(
    body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
    db: &dyn HirLazyExprDb,
) -> Option<(HirLazyExprIdx, HirLazyExprRegion)> {
    let (body, syn_expr_region) = body_with_syn_expr_region?;
    let sema_expr_region = db.sema_expr_region(syn_expr_region);
    let (hir_lazy_expr_region, hir_lazy_source_map) =
        hir_lazy_expr_region_with_source_map(db, sema_expr_region);
    let hir_lazy_source_map_data = hir_lazy_source_map.data(db);
    let body = sema_expr_region.data(db).syn_root_to_sema_expr_idx(body);
    let Some(body) = hir_lazy_source_map_data.sema_to_hir_lazy_expr_idx(body) else {
        todo!()
    };
    Some((body, hir_lazy_expr_region))
}

pub fn hir_lazy_expr_region(
    syn_expr_region: SynExprRegion,
    db: &dyn HirLazyExprDb,
) -> HirLazyExprRegion {
    let sema_expr_region = db.sema_expr_region(syn_expr_region);
    hir_lazy_expr_region_with_source_map(db, sema_expr_region).0
}
