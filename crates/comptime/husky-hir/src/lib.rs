mod db;
mod engine;
mod hir;
#[cfg(test)]
mod tests;

pub use db::*;
pub use hir::*;

use husky_coword::*;
use husky_item_path::*;

#[salsa::jar(db = HirDb)]
pub struct HirJar(ExprHirRegion);
