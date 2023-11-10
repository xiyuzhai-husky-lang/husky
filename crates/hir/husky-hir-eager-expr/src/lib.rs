#![feature(trait_upcasting)]
pub mod builder;
pub mod db;
mod expr;
pub mod helpers;
mod pattern;
mod pattern_expr;
mod region;
mod source_map;
mod stmt;
pub mod variable;

pub use self::expr::*;
pub use self::pattern::*;
pub use self::pattern_expr::*;
pub use self::region::*;
pub use self::source_map::*;
pub use self::stmt::*;

use self::builder::*;
use self::db::*;
use self::variable::*;
use husky_coword::*;
use husky_entity_path::*;
use husky_term_prelude::*;
use idx_arena::{map::ArenaMap, ordered_map::ArenaOrderedMap, Arena, ArenaIdx, ArenaIdxRange};
use salsa::DebugWithDb;
use smallvec::*;

pub trait ToHirEager {
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
