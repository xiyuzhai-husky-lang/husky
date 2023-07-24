#![feature(trait_upcasting)]
mod db;
mod decr;
mod error;
mod parser;

pub use self::db::*;
pub use self::decr::*;
pub use self::error::*;

use self::parser::*;
use husky_item_path::*;
use husky_syn_expr::*;
use husky_token::*;

#[salsa::jar(db = DecrDb)]
pub struct SynDecrJar(DeriveDecr, ty_path_decrs);
