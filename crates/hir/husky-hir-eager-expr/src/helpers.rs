use crate::*;

use husky_syn_expr::{SynExprIdx, SynExprRegion};

pub fn hir_eager_body_with_expr_region(
    body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
    db: &dyn HirEagerExprDb,
) -> Option<(HirEagerExprIdx, HirEagerExprRegion)> {
    let (body, syn_expr_region) = body_with_syn_expr_region?;
    let sema_expr_region = db.sema_expr_region(syn_expr_region);
    let (hir_eager_expr_region, hir_eager_source_map) =
        hir_eager_expr_region_with_source_map(db, sema_expr_region);
    let hir_eager_source_map_data = hir_eager_source_map.data(db);
    let body = sema_expr_region.data(db).syn_root_to_sema_expr_idx(body);
    let Some(body) = hir_eager_source_map_data.sema_to_hir_eager_expr_idx(body) else {
        todo!()
    };
    Some((body, hir_eager_expr_region))
}

pub fn hir_eager_expr_region(
    syn_expr_region: SynExprRegion,
    db: &dyn HirEagerExprDb,
) -> HirEagerExprRegion {
    let sema_expr_region = db.sema_expr_region(syn_expr_region);
    hir_eager_expr_region_with_source_map(db, sema_expr_region).0
}
