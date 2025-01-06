pub mod atom;
pub mod product;
pub mod sum;

use self::{atom::*, product::*, sum::*};
use super::*;
use smallvec::*;
use vec_like::ordered_small_vec_map::OrderedSmallVecPairMap;

#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqInumTerm<'sess> {
    Atom(VdBsqAtomInumTerm<'sess>),
    Sum(VdBsqSumInumTerm<'sess>),
    Product(VdBsqRnumTerm, VdBsqProductInumTermBase<'sess>),
}

#[enum_class::from_variants]
#[derive(Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqNonProductNumTerm<'sess> {
    Rnum(VdBsqRnumTerm),
    AtomInum(VdBsqAtomInumTerm<'sess>),
    SumInum(VdBsqSumInumTerm<'sess>),
}

impl<'sess> std::fmt::Debug for VdBsqNonProductNumTerm<'sess> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("NonProductNumTerm(")?;
        self.show_fmt(f)?;
        f.write_str(")")
    }
}

impl<'sess> VdBsqNonProductNumTerm<'sess> {
    pub fn show_fmt(self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VdBsqNonProductNumTerm::Rnum(term) => term.show_fmt(f),
            VdBsqNonProductNumTerm::AtomInum(term) => term.show_fmt(f),
            VdBsqNonProductNumTerm::SumInum(term) => term.show_fmt(f),
        }
    }
}

#[enum_class::from_variants]
#[derive(Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqNonSumInumTerm<'sess> {
    Atom(VdBsqAtomInumTerm<'sess>),
    Product(VdBsqProductInumTermBase<'sess>),
}

impl<'sess> std::fmt::Debug for VdBsqNonSumInumTerm<'sess> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("`")?;
        self.show_fmt(f)?;
        f.write_str("`")
    }
}

impl<'sess> VdBsqNonSumInumTerm<'sess> {
    pub fn show_fmt(self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VdBsqNonSumInumTerm::Atom(term) => term.show_fmt(f),
            VdBsqNonSumInumTerm::Product(term) => todo!(),
        }
    }
}

pub type VdBsqNonSumInumTerms<'sess> = SmallVec<[VdBsqNonSumInumTerm<'sess>; 4]>;

#[floated]
pub struct VdBsqInumTermFld<'sess> {
    #[return_ref]
    pub data: VdBsqInumTermData<'sess>,
}

#[enum_class::from_variants]
#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqInumTermData<'sess> {
    Atom(VdBsqInumAtomTermData),
    Sum(VdBsqInumSumTermData<'sess>),
    Product(VdBsqProductInumTermBaseData<'sess>),
}

pub type VdBsqNonProductNumTermMap<'sess, T> =
    OrderedSmallVecPairMap<VdBsqNonProductNumTerm<'sess>, T, 4>;
pub type VdBsqInumNonSumTermMap<'sess, T> =
    OrderedSmallVecPairMap<VdBsqNonSumInumTerm<'sess>, T, 4>;
pub type VdBsqInumMonomialCoefficients<'sess> = VdBsqInumNonSumTermMap<'sess, VdBsqRnumTerm>;
pub type VdBsqExponentialPowers<'sess> = VdBsqNonProductNumTermMap<'sess, VdBsqNumTerm<'sess>>;

impl<'sess> std::fmt::Debug for VdBsqInumTermFld<'sess> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("InumTermFld(`")?;
        self.data().show_fmt(f)?;
        f.write_str("`)")
    }
}

impl<'sess> VdBsqInumTermData<'sess> {
    pub fn show_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VdBsqInumTermData::Atom(term) => term.show_fmt(f),
            VdBsqInumTermData::Sum(term) => term.show_fmt(f),
            VdBsqInumTermData::Product(term) => term.show_fmt(f),
        }
    }
}
