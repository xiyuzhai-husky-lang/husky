#![feature(trait_upcasting)]
mod db;
mod entity_path;
mod error;

pub use self::db::*;
pub use self::entity_path::*;
pub use self::error::*;

use husky_entity_path::*;
use husky_ty_expectation::*;
use husky_valid_term::*;
use husky_vfs::*;

#[salsa::jar(db = ValidTypeDb)]
pub struct ValidTypeJar(ty_path_valid_ty);
