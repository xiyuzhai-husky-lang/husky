#![feature(trait_upcasting)]
pub mod builder;
pub mod db;
mod expr;
pub mod helpers;
mod pattern;
mod pattern_expr;
mod region;
pub mod source_map;
mod stmt;

pub use self::expr::*;
pub use self::pattern::*;
pub use self::pattern_expr::*;
pub use self::region::*;
pub use self::stmt::*;

use self::builder::*;
use self::db::*;
use husky_coword::*;
use husky_ethereal_term::EtherealTerm;
use idx_arena::{map::ArenaMap, ordered_map::ArenaOrderedMap, Arena, ArenaIdx, ArenaIdxRange};
use smallvec::*;

pub trait ToHirLazy {
    type Output;

    fn to_hir_lazy(&self, builder: &mut HirLazyExprBuilder) -> Self::Output;
}
