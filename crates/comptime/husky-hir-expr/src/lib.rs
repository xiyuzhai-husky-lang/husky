pub mod db;
mod region;
mod source_map;

pub use self::region::*;
pub use self::source_map::*;

use self::db::*;

pub type HirExprIdx = ();

#[derive(Debug, PartialEq, Eq)]
pub enum HirExpr {}

#[derive(Debug, PartialEq, Eq)]
pub enum HirStmt {}
