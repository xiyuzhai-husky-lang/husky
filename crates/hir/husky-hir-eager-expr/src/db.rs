use crate::*;

#[salsa::jar(db = HirEagerExprDb)]
pub struct HirEagerExprJar(
    HirEagerExprRegion,
    HirEagerExprSourceMap,
    hir_eager_expr_region_with_source_map,
);
