#![feature(trait_upcasting)]
pub mod db;
pub mod symbol;
pub mod template_argument;
// pub mod template_parameter;
#[cfg(test)]
mod tests;
pub mod trai;
pub mod ty;
pub mod ty_constant;

pub use self::symbol::*;
pub use self::template_argument::*;
pub use self::ty::*;
pub use self::ty_constant::*;

use self::db::*;
use husky_entity_path::*;
use husky_hir_prelude::*;
use smallvec::*;
