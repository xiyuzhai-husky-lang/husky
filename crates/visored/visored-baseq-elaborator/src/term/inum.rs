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
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqNonProductNumTerm<'sess> {
    Rnum(VdBsqRnumTerm),
    AtomInum(VdBsqAtomInumTerm<'sess>),
    SumInum(VdBsqSumInumTerm<'sess>),
}

#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqNonSumInumTerm<'sess> {
    Atom(VdBsqAtomInumTerm<'sess>),
    Product(VdBsqRnumTerm, VdBsqProductInumTermBase<'sess>),
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
pub type VdBsqExponentials<'sess> = VdBsqNonProductNumTermMap<'sess, VdBsqNumTerm<'sess>>;
