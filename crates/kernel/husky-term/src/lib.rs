#![doc = include_str!("../README.md")]
// #![deny(unsafe_code, missing_docs, clippy::unwrap_used)]

mod context;
mod db;
mod error;
mod menu;
mod rewrite;
mod term;
#[cfg(test)]
mod tests;

pub use context::*;
pub use db::*;
pub use error::*;
pub use menu::*;
pub use rewrite::*;
pub use term::*;

use husky_entity_path::EntityPath;
use husky_print_utils::p;
use husky_vfs::*;
use husky_word::Identifier;

#[salsa::jar(db = TermDb)]
pub struct TermJar(
    TermSymbol,
    TermCurry,
    TermRitchie,
    TermAbstraction,
    TermApplication,
    TermSubentity,
    TermAsTraitSubentity,
    TermTraitConstraint,
    TermInteger128,
    TermInteger256,
    TermNaturalNumber,
    term_menu,
);

#[test]
fn term_size_works() {
    assert_eq!(
        std::mem::size_of::<Term>(),
        2 * std::mem::size_of::<usize>()
    )
}
