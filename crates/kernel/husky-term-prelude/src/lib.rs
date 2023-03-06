mod curry;
mod db;
mod literal;

pub use self::curry::*;
pub use self::db::*;
pub use self::literal::*;

use husky_entity_path::*;

#[salsa::jar(db = TermPreludeDb)]
pub struct TermPreludeJar(
    PreciseTermInteger128,
    PreciseTermInteger256,
    PreciseTermNaturalNumber,
);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TermRitchieKind {
    Fp,
    Fn,
    FnMut,
}
