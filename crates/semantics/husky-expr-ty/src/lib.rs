#![feature(if_let_guard)]
#![feature(trait_upcasting)]
#![feature(const_trait_impl)]
mod db;
mod engine;
mod error;
mod info;
mod region;
#[cfg(test)]
mod tests;

pub use self::db::*;
pub use self::error::*;
pub use self::info::*;
pub use self::region::*;

use self::engine::*;
use either::*;
use husky_defn::*;
use husky_entity_path::*;
use husky_entity_taxonomy::*;
use husky_entity_tree::*;
use husky_ethereal_term::*;
use husky_expr::*;
use husky_fluffy_term::*;
use husky_signature::*;
use husky_term_prelude::*;
use salsa::DebugWithDb;
use smallvec::*;

#[cfg(test)]
use tests::*;

#[salsa::jar(db = ExprTypeDb)]
pub struct ExprTypeJar(expr_ty_region);
