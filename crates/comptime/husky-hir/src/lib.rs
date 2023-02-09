mod db;
mod hir;
#[cfg(test)]
mod tests;

pub use db::*;
pub use hir::*;

use husky_entity_path::*;
use husky_word::*;
#[cfg(test)]
use tests::*;

#[salsa::jar(db = HirDb)]
pub struct HirJar(ExprHirRegion);
