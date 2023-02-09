mod db;
mod engine;
mod hir;
#[cfg(test)]
mod tests;
mod transpiler;

pub use db::*;
pub use hir::*;

#[cfg(test)]
use tests::*;
use transpiler::*;

#[salsa::jar(db = CHirDb)]
pub struct CHirJar();
