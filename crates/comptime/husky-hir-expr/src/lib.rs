#![feature(trait_upcasting)]
pub mod builder;
pub mod db;
pub mod helpers;
mod region;
pub mod source_map;

pub use self::region::*;

use self::builder::*;
use self::db::*;
use husky_hir_eager_expr::*;
use husky_hir_lazy_expr::*;
use husky_sema_expr::SemaExprIdx;
use husky_syn_expr::SynExprIdx;

pub trait ToHir {
    type Output;

    fn to_hir(&self, builder: &mut HirExprBuilder) -> Self::Output;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[enum_class::from_variants]
pub enum HirExprIdx {
    Eager(HirEagerExprIdx),
    Lazy(HirLazyExprIdx),
}

impl ToHir for SemaExprIdx {
    type Output = HirExprIdx;

    fn to_hir(&self, builder: &mut HirExprBuilder) -> Self::Output {
        match builder {
            HirExprBuilder::Eager(builder) => self.to_hir_eager(builder).into(),
            HirExprBuilder::Lazy(builder) => self.to_hir_lazy(builder).into(),
        }
    }
}
