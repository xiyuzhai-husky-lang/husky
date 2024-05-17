#![feature(impl_trait_in_assoc_type)]
#![feature(result_flattening)]
#![doc = include_str ! ("../README.md")]
#![feature(let_chains)]
#![feature(if_let_guard)]
mod conversion;
mod db;
mod error;
pub mod fmt;
mod helpers;
pub mod instantiation;
pub mod jar;
mod menu;
mod rewrite;
mod template_parameter;
pub mod term;
#[cfg(test)]
mod tests;
mod ty;

pub use self::db::*;
pub use self::error::*;
pub use self::menu::*;
pub use self::rewrite::substitute::*;
pub use self::template_parameter::*;
pub use self::ty::*;

use self::jar::EthTermJar as Jar;
use self::term::EthTerm;
use either::*;
use husky_dec_term::{jar::DecTermDb, term::*, *};
use husky_entity_path::*;
use husky_term_prelude::*;
use husky_vfs::*;
use salsa::DebugWithDb;
use smallvec::*;
