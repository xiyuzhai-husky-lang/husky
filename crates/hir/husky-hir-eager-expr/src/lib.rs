pub mod builder;
pub mod coersion;
pub mod db;
pub mod expr;
pub mod helpers;
mod pattern;
mod pattern_expr;
mod region;
mod source_map;
pub mod stmt;
pub mod symbol;

pub use self::expr::*;
pub use self::pattern::*;
pub use self::pattern_expr::*;
pub use self::region::*;
pub use self::source_map::*;
pub use self::stmt::*;

use self::builder::*;
use self::db::*;
use husky_coword::*;
use husky_entity_path::*;
use husky_term_prelude::*;
use idx_arena::{map::ArenaMap, ordered_map::ArenaOrderedMap, Arena, ArenaIdx, ArenaIdxRange};

use smallvec::*;

pub(crate) trait ToHirEager {
    type Output;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output;
}

impl<T> ToHirEager for Option<T>
where
    T: ToHirEager,
{
    type Output = Option<T::Output>;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        self.as_ref().map(|t| t.to_hir_eager(builder))
    }
}

impl<T> ToHirEager for [T]
where
    T: ToHirEager,
{
    type Output = Vec<T::Output>;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        self.iter().map(|t| t.to_hir_eager(builder)).collect()
    }
}
