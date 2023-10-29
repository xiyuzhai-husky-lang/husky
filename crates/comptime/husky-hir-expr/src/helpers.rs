use husky_syn_expr::SynExprRegion;

use crate::{source_map::HirExprSourceMap, *};

pub fn build_body(
    body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
    db: &dyn HirExprDb,
) -> Option<(HirExprIdx, HirExprRegion, HirExprSourceMap)> {
    let (body, syn_expr_region) = body_with_syn_expr_region?;
    let mut builder = HirExprBuilder::new(db, syn_expr_region);
    let body = builder.build_hir_expr(body);
    let (hir_expr_region, hir_expr_source_map) = builder.finish();
    Some((body, hir_expr_region, hir_expr_source_map))
}
