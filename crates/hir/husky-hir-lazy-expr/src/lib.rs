#![feature(try_trait_v2)]
pub mod builder;
pub mod expr;
pub mod helpers;
pub mod jar;
mod pattern;
mod pattern_expr;
mod region;
pub mod source_map;
pub mod stmt;
pub mod var_deps;
pub mod variable;

pub use self::expr::*;
pub use self::pattern::*;
pub use self::pattern_expr::*;
pub use self::region::*;
pub use self::stmt::*;

use self::builder::*;
use self::jar::HirLazyExprJar as Jar;
use self::jar::*;
use self::variable::*;
use husky_coword::*;
use husky_eth_term::term::EthTerm;
use idx_arena::{map::ArenaMap, ordered_map::ArenaOrderedMap, Arena, ArenaIdx, ArenaIdxRange};
use smallvec::*;

pub(crate) trait ToHirLazy {
    type Output;

    fn to_hir_lazy(&self, builder: &mut HirLazyExprBuilder) -> Self::Output;
}
