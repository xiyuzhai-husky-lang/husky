#![feature(if_let_guard)]
pub mod db;
pub mod indirections;
pub mod instantiation;
pub mod lifetime;
pub mod menu;
pub mod place_contract_site;
pub mod quary;
pub mod svar;
pub mod template_argument;
#[cfg(test)]
mod tests;
pub mod trai;
pub mod ty;
pub mod ty_constant;

pub use self::svar::*;
pub use self::template_argument::*;
pub use self::ty::*;
pub use self::ty_constant::*;

use self::db::*;
use husky_entity_path::*;
use smallvec::*;
