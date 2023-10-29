use crate::*;
use husky_sema_expr::SemaExprIdx;
use husky_syn_expr::{SynExprIdx, SynExprRegion};

pub fn build_eager_body(
    body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
    db: &dyn HirEagerExprDb,
) -> Option<(HirEagerExprIdx, HirEagerExprRegion, HirEagerExprSourceMap)> {
    let (body, syn_expr_region) = body_with_syn_expr_region?;
    let mut builder = HirEagerExprBuilder::new(db, syn_expr_region);
    let body = builder.build_hir_eager_expr(body);
    let (hir_eager_expr_region, hir_eager_expr_source_map) = builder.finish();
    Some((body, hir_eager_expr_region, hir_eager_expr_source_map))
}
