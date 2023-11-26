use crate::{source_map::HirLazyExprSourceMap, *};

#[salsa::jar(db = HirLazyExprDb)]
pub struct HirLazyExprJar(
    HirLazyExprRegion,
    HirLazyExprSourceMap,
    hir_lazy_expr_region_with_source_map,
    crate::helpers::control_flow::hir_lazy_expr_region_control_flow,
);
