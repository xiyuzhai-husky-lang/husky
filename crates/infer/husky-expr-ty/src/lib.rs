#![feature(trait_upcasting)]
#![feature(const_trait_impl)]
mod db;
mod engine;
mod error;
mod local_term;
mod region;
#[cfg(test)]
mod tests;

pub use db::*;
pub use error::*;
pub use local_term::*;
pub use region::*;

use engine::*;
use husky_defn::*;
use husky_entity_path::*;
use husky_entity_taxonomy::*;
use husky_entity_tree::*;
use husky_expr::*;
use husky_signature::*;
use husky_term::*;
use husky_ty::*;
use salsa::DebugWithDb;

#[cfg(test)]
use tests::*;

#[salsa::jar(db = ExprTypeDb)]
pub struct ExprTypeJar(expr_ty_region);
