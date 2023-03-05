mod db;
mod entity_path;
mod error;

pub use self::db::*;
pub use self::entity_path::*;
pub use self::error::*;

use husky_entity_path::*;
use husky_precise_term::*;
use husky_ty_expectation::*;
use husky_vfs::*;

#[salsa::jar(db = PreciseTypeDb)]
pub struct PreciseTypeJar(ty_path_precise_ty);
