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
    Atom(VdBsqAtomComnumTerm<'sess>),
    Sum(VdBsqSumComnumTerm<'sess>),
    Product(VdBsqLitnumTerm<'sess>, VdBsqProductBase<'sess>),
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
            VdBsqComnumTerm::Product(litnum, term) => {
                debug_assert!(!litnum.is_zero());
                if litnum.is_one() {
                    debug_assert!(!term.data().exponentials().is_empty());
                    term.show_fmt(precedence_range, f)
                } else {
                    fn show_product_fmt_inner<'sess>(
                        litnum: VdBsqLitnumTerm<'sess>,
                        term: VdBsqProductBase<'sess>,
                        f: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        litnum.show_fmt(VdPrecedenceRange::MUL_DIV_LEFT, f)?;
                        match term.exponentials().data()[0].0 {
                            VdBsqNonProductNumTerm::Litnum(_) => f.write_str(" × ")?,
                            VdBsqNonProductNumTerm::AtomComnum(_)
                            | VdBsqNonProductNumTerm::SumComnum(_) => (),
                        }
                        term.show_fmt(VdPrecedenceRange::MUL_DIV_RIGHT, f)
                    }

                    if precedence_range.contains(VdPrecedence::MUL_DIV) {
                        show_product_fmt_inner(litnum, term, f)
                    } else {
                        f.write_str("(")?;
                        show_product_fmt_inner(litnum, term, f)?;
                        f.write_str(")")
                    }
                }
            }
        }
    }
}

#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqNonProductNumTerm<'sess> {
    Litnum(VdBsqLitnumTerm<'sess>),
    AtomComnum(VdBsqAtomComnumTerm<'sess>),
    SumComnum(VdBsqSumComnumTerm<'sess>),
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

#[floated]
pub struct VdBsqComnumTermFld<'sess> {
    #[return_ref]
    pub data: VdBsqComnumTermData<'sess>,
}

#[enum_class::from_variants]
#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqComnumTermData<'sess> {
    Atom(VdBsqComnumAtomTermData),
    Sum(VdBsqComnumSumTermData<'sess>),
    // TODO: maybe remove this???
    Product(VdBsqProductComnumTermBaseData<'sess>),
}

pub type VdBsqNonProductNumTermMap<'sess, T> =
    OrderedSmallVecPairMap<VdBsqNonProductNumTerm<'sess>, T, 4>;
pub type VdBsqNonSumComnumTermMap<'sess, T> = OrderedSmallVecPairMap<VdBsqProductBase<'sess>, T, 4>;
pub type VdBsqMonomialCoefficients<'sess> = VdBsqNonSumComnumTermMap<'sess, VdBsqLitnumTerm<'sess>>;
pub type VdBsqExponentialPowers<'sess> = VdBsqNonProductNumTermMap<'sess, VdBsqNumTerm<'sess>>;
pub type VdBsqExponentialPowersRef<'a, 'sess> =
    &'a [(VdBsqNonProductNumTerm<'sess>, VdBsqNumTerm<'sess>)];
pub type VdBsqExponentialParts<'sess> = Vec<(VdBsqNonProductNumTerm<'sess>, VdBsqNumTerm<'sess>)>;

impl<'sess> std::fmt::Debug for VdBsqComnumTermFld<'sess> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ComnumTermFld(`")?;
        self.data().show_fmt(VdPrecedenceRange::Any, f)?;
        f.write_str("`)")
    }
}

impl<'sess> VdBsqComnumTermData<'sess> {
    pub fn show_fmt(
        &self,
        precedence_range: VdPrecedenceRange,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            VdBsqComnumTermData::Atom(term) => term.show_fmt(precedence_range, f),
            VdBsqComnumTermData::Sum(term) => term.show_fmt(precedence_range, f),
            VdBsqComnumTermData::Product(term) => term.show_fmt(precedence_range, f),
        }
    }
}

impl<'sess> VdBsqComnumTerm<'sess> {
    pub fn neg(self, db: &'sess FloaterDb) -> VdBsqComnumTerm<'sess> {
        match self {
            VdBsqComnumTerm::Atom(term) => term.neg(db).into(),
            VdBsqComnumTerm::Sum(term) => term.neg(db).into(),
            VdBsqComnumTerm::Product(litnum, term) => {
                VdBsqComnumTerm::Product(litnum.neg(db), term).into()
            }
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
            VdBsqComnumTerm::Atom(term) => term.mul128(rhs, db),
            VdBsqComnumTerm::Sum(term) => term.mul128(rhs, db),
            VdBsqComnumTerm::Product(litnum, term) => {
                VdBsqComnumTerm::Product(litnum.mul128(rhs, db), term).into()
            }
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
            VdBsqComnumTerm::Product(litnum, term) => {
                Some(VdBsqComnumTerm::Product(litnum.div(rhs, db).unwrap(), term).into())
            }
        }
    }
}
