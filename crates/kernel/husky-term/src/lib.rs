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
mod ty;

pub use self::context::*;
pub use self::db::*;
pub use self::error::*;
pub use self::menu::*;
pub use self::rewrite::*;
pub use self::term::*;
pub use self::ty::*;

use either::*;
use husky_entity_path::*;
use husky_print_utils::p;
use husky_raw_term::*;
use husky_term_prelude::*;
use husky_vfs::*;
use husky_word::Identifier;

#[salsa::jar(db = TermDb)]
pub struct TermJar(
    TermSymbol,
    TermSymbols,
    term_curry_symbols,
    term_ritchie_symbols,
    term_application_symbols,
    // entity path
    ty_ontology_path_ty_unchecked,
    ty_constructor_path_ty_unchecked,
    trai_path_ty_unchecked,
    form_path_ty_unchecked,
    // curry
    TermCurry,
    TermRitchie,
    TermAbstraction,
    // application
    TermApplication,
    application_expansion_salsa,
    term_from_raw_application,
    ApplicationArguments,
    // subentity
    TermSubentity,
    TermAsTraitSubentity,
    TermTraitConstraint,
    term_menu,
    // only use this inside crate::context::entry
    is_ty_path_lifetime_ty,
);
