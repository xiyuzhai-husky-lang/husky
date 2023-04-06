#![feature(trait_upcasting)]
mod category;
mod contract;
mod curry;
mod db;
mod entity_path;
mod error;
mod literal;
mod pattern;
mod universe;

pub use self::category::*;
pub use self::contract::*;
pub use self::curry::*;
pub use self::db::*;
pub use self::entity_path::*;
pub use self::error::*;
pub use self::literal::*;
pub use self::pattern::*;
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
