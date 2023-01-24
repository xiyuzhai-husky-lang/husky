mod db;
mod engine;
mod local_term;
mod region;
#[cfg(test)]
mod tests;

pub use db::*;
pub use region::*;

use husky_entity_path::*;
use husky_entity_taxonomy::*;
use husky_expr::*;
use husky_signature::*;
use husky_term::*;
use husky_ty::*;

#[cfg(test)]
use tests::*;

#[salsa::jar(db = ExprTypeDb)]
pub struct ExprTypeJar(expr_ty_region);
