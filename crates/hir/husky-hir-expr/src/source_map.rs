use husky_hir_eager_expr::HirEagerExprSourceMap;
use husky_hir_lazy_expr::source_map::HirLazyExprSourceMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[enum_class::from_variants]
pub enum HirExprSourceMap {
    Eager(HirEagerExprSourceMap),
    Lazy(HirLazyExprSourceMap),
}
