pub mod atom;
pub mod helpers;
pub mod product;
pub mod sum;

use self::{atom::*, product::*, sum::*};
use super::*;
use smallvec::*;
use vec_like::ordered_small_vec_map::OrderedSmallVecPairMap;
use visored_opr::precedence::VdPrecedence;

#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqComnumTerm<'sess> {
    Atom(VdBsqAtomTerm<'sess>),
    Sum(VdBsqSumTerm<'sess>),
    Product(VdBsqProductTerm<'sess>),
}

impl<'sess> VdBsqComnumTerm<'sess> {
    pub fn show_fmt(
        self,
        precedence_range: VdPrecedenceRange,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            VdBsqComnumTerm::Atom(term) => term.show_fmt(precedence_range, f),
            VdBsqComnumTerm::Sum(term) => term.show_fmt(precedence_range, f),
            VdBsqComnumTerm::Product(term) => term.show_fmt(precedence_range, f),
        }
    }
}

#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqNonProductNumTerm<'sess> {
    Litnum(VdBsqLitnumTerm<'sess>),
    AtomComnum(VdBsqAtomTerm<'sess>),
    SumComnum(VdBsqSumTerm<'sess>),
}

impl<'sess> VdBsqNonProductNumTerm<'sess> {
    pub fn show_fmt(
        self,
        precedence_range: VdPrecedenceRange,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            VdBsqNonProductNumTerm::Litnum(term) => term.show_fmt(precedence_range, f),
            VdBsqNonProductNumTerm::AtomComnum(term) => term.show_fmt(precedence_range, f),
            VdBsqNonProductNumTerm::SumComnum(term) => term.show_fmt(precedence_range, f),
        }
    }

    pub fn outer_precedence(&self) -> VdPrecedence {
        match self {
            VdBsqNonProductNumTerm::Litnum(term) => term.outer_precedence(),
            VdBsqNonProductNumTerm::AtomComnum(term) => term.outer_precedence(),
            VdBsqNonProductNumTerm::SumComnum(term) => VdPrecedence::ADD_SUB,
        }
    }
}

pub type VdBsqNonSumComnumTerms<'sess> = SmallVec<[VdBsqProductBase<'sess>; 4]>;
pub type VdBsqNonProductNumTermMap<'sess, T> =
    OrderedSmallVecPairMap<VdBsqNonProductNumTerm<'sess>, T, 4>;
pub type VdBsqNonSumComnumTermMap<'sess, T> = OrderedSmallVecPairMap<VdBsqProductBase<'sess>, T, 4>;
pub type VdBsqMonomialCoefficients<'sess> = VdBsqNonSumComnumTermMap<'sess, VdBsqLitnumTerm<'sess>>;
pub type VdBsqExponentialPowers<'sess> = VdBsqNonProductNumTermMap<'sess, VdBsqNumTerm<'sess>>;
pub type VdBsqExponentialPowersRef<'a, 'sess> =
    &'a [(VdBsqNonProductNumTerm<'sess>, VdBsqNumTerm<'sess>)];
pub type VdBsqExponentialParts<'sess> = Vec<(VdBsqNonProductNumTerm<'sess>, VdBsqNumTerm<'sess>)>;

impl<'sess> VdBsqComnumTerm<'sess> {
    pub fn neg(self, db: &'sess FloaterDb) -> VdBsqComnumTerm<'sess> {
        match self {
            VdBsqComnumTerm::Atom(term) => term.neg(db).into(),
            VdBsqComnumTerm::Sum(term) => term.neg(db).into(),
            VdBsqComnumTerm::Product(product) => product
                .with_litnum_factor_update(|litnum| litnum.neg(db))
                .into(),
        }
    }

    pub fn mul128(self, rhs: i128, db: &'sess FloaterDb) -> VdBsqNumTerm<'sess> {
        if rhs == 0 {
            return VdBsqNumTerm::ZERO;
        }
        if rhs == 1 {
            return self.into();
        }
        match self {
            VdBsqComnumTerm::Atom(term) => term.mul_litnum(rhs, db),
            VdBsqComnumTerm::Sum(term) => term.mul128(rhs, db),
            VdBsqComnumTerm::Product(product) => product
                .with_litnum_factor_update(|litnum| litnum.mul128(rhs, db))
                .into(),
        }
    }

    pub fn div_litnum(
        self,
        rhs: VdBsqLitnumTerm<'sess>,
        db: &'sess FloaterDb,
    ) -> Option<VdBsqComnumTerm<'sess>> {
        if rhs.is_zero() {
            return None;
        }
        if rhs.is_one() {
            return Some(self.into());
        }
        match self {
            VdBsqComnumTerm::Atom(slf) => Some(slf.div_litnum(rhs, db).unwrap().into()),
            VdBsqComnumTerm::Sum(slf) => Some(slf.div_litnum(rhs, db).unwrap()),
            VdBsqComnumTerm::Product(product) => Some(
                product
                    .with_litnum_factor_update(|litnum| litnum.div(rhs, db).unwrap())
                    .into(),
            ),
        }
    }
}
