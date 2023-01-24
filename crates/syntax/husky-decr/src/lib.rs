mod db;
mod decr;

pub use db::*;
pub use decr::*;

use husky_entity_path::*;
use husky_expr::*;
use husky_token::*;

#[salsa::jar(db = DecrDb)]
pub struct DecrJar(DeriveDecr);
