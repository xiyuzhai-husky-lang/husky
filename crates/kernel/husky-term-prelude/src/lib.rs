#![feature(trait_upcasting)]
mod category;
mod curry;
mod db;
mod entity_path;
mod error;
mod liason;
mod literal;
mod universe;

pub use self::category::*;
pub use self::curry::*;
pub use self::db::*;
pub use self::entity_path::*;
pub use self::error::*;
pub use self::liason::*;
pub use self::literal::*;
pub use self::universe::*;

use husky_entity_path::*;

#[salsa::jar(db = TermPreludeDb)]
pub struct TermPreludeJar(
    TermI32Literal,
    TermI64Literal,
    TermI128Literal,
    TermI256Literal,
    TermISizeLiteral,
    TermU32Literal,
    TermU64Literal,
    TermU128Literal,
    TermU256Literal,
    TermUSizeLiteral,
    TermR32Literal,
    TermR64Literal,
    TermR128Literal,
    TermR256Literal,
    TermRSizeLiteral,
    TermF32Literal,
    TermF64Literal,
    TermNatLiteral,
);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TermRitchieKind {
    FnType,
    FnTrait,
    FnMutTrait,
}
