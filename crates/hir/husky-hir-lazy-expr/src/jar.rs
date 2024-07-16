use crate::{source_map::HirLazyExprSourceMap, *};

#[salsa::jar]
pub struct HirLazyExprJar(
    HirLazyExprRegion,
    HirLazyExprSourceMap,
    hir_lazy_expr_region_with_source_map,
    crate::var_deps::hir_lazy_expr_var_deps_region,
    crate::helpers::control_flow::hir_lazy_expr_region_control_flow,
);
