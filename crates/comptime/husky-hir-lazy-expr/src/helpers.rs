use crate::*;
use husky_sema_expr::SemaExprIdx;
use husky_syn_expr::{SynExprIdx, SynExprRegion};

pub fn build_lazy_body(
    body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
    db: &dyn HirLazyExprDb,
) -> Option<(HirLazyExprIdx, HirLazyExprRegion)> {
    let (body, syn_expr_region) = body_with_syn_expr_region?;
    let mut builder = HirLazyExprBuilder::new(db, syn_expr_region);
    let body = builder.build_hir_lazy_expr(body);
    let hir_expr_region = builder.finish();
    Some((body, hir_expr_region))
}
