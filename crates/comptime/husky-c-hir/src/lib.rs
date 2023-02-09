mod db;
mod hir;
#[cfg(test)]
mod tests;

pub use db::*;
pub use hir::*;
#[cfg(test)]
use tests::*;

#[salsa::jar(db = CHirDb)]
pub struct CHirJar();
