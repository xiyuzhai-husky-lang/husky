#![feature(trait_upcasting)]
mod category;
mod curry;
mod db;
mod entity_path;
mod error;
mod literal;
mod universe;

pub use self::category::*;
pub use self::curry::*;
pub use self::db::*;
pub use self::entity_path::*;
pub use self::error::*;
pub use self::literal::*;
pub use self::universe::*;

use husky_entity_path::*;

#[salsa::jar(db = TermPreludeDb)]
pub struct TermPreludeJar(TermInteger128, TermInteger256, TermNaturalNumber);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TermRitchieKind {
    Fp,
    Fn,
    FnMut,
}
