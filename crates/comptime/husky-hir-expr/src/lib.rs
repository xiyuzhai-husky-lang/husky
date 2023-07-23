pub mod db;
mod region;

pub use self::region::*;

pub type HirExprIdx = ();

#[derive(Debug, PartialEq, Eq)]
pub enum HirExpr {}

#[derive(Debug, PartialEq, Eq)]
pub enum HirStmt {}
