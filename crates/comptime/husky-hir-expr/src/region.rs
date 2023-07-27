use crate::*;

#[enum_class::from_variants]
pub enum HirExprRegion {
    Eager(HirEagerExprRegion),
    Lazy(HirLazyExprRegion),
}
