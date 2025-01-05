pub mod engine;
pub mod tracker;

use husky_sha_utils::Sha256Output;
use idx_arena::{Arena, ArenaIdx};
use vec_like::ordered_small_vec_map::OrderedSmallVecPairMap;
use visored_term::term::literal::VdLiteral;

#[enum_class::from_variants]
#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum Term {
    Rational(RationalTerm),
    Irrational(IrrationalTerm),
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum RationalTerm {
    Int128(i128),
    BigInt(/* TODO */),
    Rat128(i128, u128),
}

impl std::ops::AddAssign for RationalTerm {
    fn add_assign(&mut self, rhs: Self) {
        match self {
            RationalTerm::Int128(slf) => match rhs {
                RationalTerm::Int128(rhs) => match slf.checked_add(rhs) {
                    Some(sum) => *self = RationalTerm::Int128(sum),
                    None => todo!(),
                },
                RationalTerm::BigInt() => todo!(),
                RationalTerm::Rat128(_, _) => todo!(),
            },
            RationalTerm::BigInt() => todo!(),
            RationalTerm::Rat128(_, _) => todo!(),
        }
    }
}

impl RationalTerm {
    pub const ZERO: Self = Self::Int128(0);
    pub const ONE: Self = Self::Int128(1);
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum IrrationalTermData {
    Atom,
    Product {
        literal: RationalTerm,
        irrational_atom_exponentials: IrrationalAtomExponentials,
    },
    Sum {
        constant_term: RationalTerm,
        irrational_monomial_coefficients: IrrationalMonomialCoefficients,
    },
    Variable(ArenaIdx<visored_mir_expr::symbol::local_defn::VdMirSymbolLocalDefnData>),
}

pub type IrrationalMonomialCoefficients = IrrationalTermMap<RationalTerm>;
pub type IrrationalAtomExponentials = IrrationalTermMap<Term>;

pub struct IrrationalTermEntry {
    data: IrrationalTermData,
    sha256: Sha256Output,
}

pub type IrrationalTerm = ArenaIdx<IrrationalTermEntry>;
pub type IrrationalTermArena = Arena<IrrationalTermEntry>;
pub type IrrationalTermMap<T> = OrderedSmallVecPairMap<IrrationalTerm, T, 4>;
