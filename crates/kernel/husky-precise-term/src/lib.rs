#![doc = include_str!("../README.md")]
#![feature(trait_upcasting)]
#![feature(let_chains)]
// #![deny(unsafe_code, missing_docs, clippy::unwrap_used)]

mod context;
mod db;
mod error;
mod menu;
mod rewrite;
mod term;
#[cfg(test)]
mod tests;

pub use self::context::*;
pub use self::db::*;
pub use self::error::*;
pub use self::menu::*;
pub use self::rewrite::*;
pub use self::term::*;

use husky_entity_path::EntityPath;
use husky_print_utils::p;
use husky_raw_ty::*;
use husky_term_attrs::*;
use husky_vfs::*;
use husky_word::Identifier;

#[salsa::jar(db = PreciseTermDb)]
pub struct PreciseTermJar(
    PreciseTermSymbol,
    PreciseTermCurry,
    PreciseTermRitchie,
    PreciseTermAbstraction,
    PreciseTermApplication,
    PreciseTermSubentity,
    PreciseTermAsTraitSubentity,
    PreciseTermTraitConstraint,
    PreciseTermInteger128,
    PreciseTermInteger256,
    PreciseTermNaturalNumber,
    precise_term_menu,
    // only use this inside crate::context::entry
    is_ty_path_lifetime_ty,
);
