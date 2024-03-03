pub mod helpers;
pub mod jar;
mod region;
pub mod source_map;

pub use self::region::*;

use husky_hir_eager_expr::*;
use husky_hir_lazy_expr::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[enum_class::from_variants]
pub enum HirExprIdx {
    Eager(HirEagerExprIdx),
    Lazy(HirLazyExprIdx),
}
