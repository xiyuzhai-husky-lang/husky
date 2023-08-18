pub mod builder;
pub mod db;
mod expr;
mod region;
mod stmt;

pub use self::expr::*;
pub use self::region::*;

use self::builder::*;
use self::db::*;
use idx_arena::{map::ArenaMap, ordered_map::ArenaOrderedMap, Arena, ArenaIdx, ArenaIdxRange};

pub trait ToHirLazy {
    type Output;

    fn to_hir_lazy(&self, builder: &mut HirLazyExprBuilder) -> Self::Output;
}
