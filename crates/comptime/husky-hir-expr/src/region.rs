use crate::*;

#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HirExprRegion {
    Eager(HirEagerExprRegion),
    Lazy(HirLazyExprRegion),
}
