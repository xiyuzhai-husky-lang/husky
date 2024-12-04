mod engine;

use berserk::*;
use db::BerserkerDb;
use husky_sha_utils::Sha256Output;
use idx_arena::{Arena, ArenaIdx};
use vec_like::ordered_small_vec_map::OrderedSmallVecPairMap;
use visored_term::term::literal::VdLiteral;

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum Term {
    Literal(LiteralTerm),
    NonLiteral(NonLiteralTerm),
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum LiteralTerm {
    Nat128(i128),
    BigNat(/* TODO */),
    Rat128(i128, u128),
}

impl LiteralTerm {
    pub const ZERO: Self = Self::Nat128(0);
    pub const ONE: Self = Self::Nat128(1);
}

type NonLiteralAtomExponentials = NonLiteralTermMap<NonLiteralTerm>;

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum NonLiteralTermData {
    Atom,
    Product {
        literal: LiteralTerm,
        nonliteral_atom_exponentials: NonLiteralAtomExponentials,
    },
    Sum {
        nonliteral_monomial_coefficients: Vec<NonLiteralTerm>,
        constant_term: LiteralTerm,
    },
}

pub struct NonLiteralTermEntry {
    data: NonLiteralTermData,
    sha256: Sha256Output,
}

pub type NonLiteralTerm = ArenaIdx<NonLiteralTermEntry>;
pub type NonLiteralTermArena = Arena<NonLiteralTermEntry>;
pub type NonLiteralTermMap<T> = OrderedSmallVecPairMap<NonLiteralTerm, T, 4>;
