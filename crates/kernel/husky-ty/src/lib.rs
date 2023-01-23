#![feature(trait_upcasting)]
mod db;
mod error;
#[cfg(test)]
mod tests;

pub use db::*;
pub use error::*;

use husky_entity_path::*;
use husky_entity_taxonomy::*;
use husky_signature::*;
use husky_term::*;

#[salsa::jar(db=TypeDb)]
pub struct TypeJar(ty_entity_ty, trai_entity_ty);
