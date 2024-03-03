use crate::*;

#[salsa::jar]
pub struct HirEagerExprJar(
    HirEagerExprRegion,
    HirEagerExprSourceMap,
    hir_eager_expr_region_with_source_map,
);
