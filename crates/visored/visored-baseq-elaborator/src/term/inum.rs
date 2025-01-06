pub mod atom;
pub mod product;
pub mod sum;

use self::{atom::*, product::*, sum::*};
use super::*;
use smallvec::*;
use vec_like::ordered_small_vec_map::OrderedSmallVecPairMap;
use visored_opr::precedence::VdPrecedence;

#[enum_class::from_variants]
#[derive(Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqInumTerm<'sess> {
    Atom(VdBsqAtomInumTerm<'sess>),
    Sum(VdBsqSumInumTerm<'sess>),
    Product(VdBsqRnumTerm, VdBsqProductInumTermBase<'sess>),
}

impl<'sess> std::fmt::Debug for VdBsqInumTerm<'sess> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("InumTerm(`")?;
        self.show_fmt(VdPrecedenceRange::Any, f)?;
        f.write_str("`)")
    }
}

impl<'sess> VdBsqInumTerm<'sess> {
    pub fn show_fmt(
        self,
        precedence_range: VdPrecedenceRange,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            VdBsqInumTerm::Atom(term) => term.show_fmt(precedence_range, f),
            VdBsqInumTerm::Sum(term) => term.show_fmt(precedence_range, f),
            VdBsqInumTerm::Product(rnum, term) => {
                debug_assert!(!rnum.is_zero());
                if rnum.is_one() {
                    debug_assert!(!term.data().exponentials().is_empty());
                    term.show_fmt(precedence_range, f)
                } else {
                    fn show_product_fmt_inner<'sess>(
                        rnum: VdBsqRnumTerm,
                        term: VdBsqProductInumTermBase<'sess>,
                        f: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        rnum.show_fmt(VdPrecedenceRange::MUL_DIV_LEFT, f)?;
                        match term.exponentials()[0].0 {
                            VdBsqNonProductNumTerm::Rnum(_) => f.write_str(" × ")?,
                            VdBsqNonProductNumTerm::AtomInum(_)
                            | VdBsqNonProductNumTerm::SumInum(_) => (),
                        }
                        term.show_fmt(VdPrecedenceRange::MUL_DIV_RIGHT, f)
                    }

                    if precedence_range.contains(VdPrecedence::MUL_DIV) {
                        show_product_fmt_inner(rnum, term, f)
                    } else {
                        f.write_str("(")?;
                        show_product_fmt_inner(rnum, term, f)?;
                        f.write_str(")")
                    }
                }
            }
        }
    }
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
        self.show_fmt(VdPrecedenceRange::Any, f)?;
        f.write_str(")")
    }
}

impl<'sess> VdBsqNonProductNumTerm<'sess> {
    pub fn show_fmt(
        self,
        precedence_range: VdPrecedenceRange,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            VdBsqNonProductNumTerm::Rnum(term) => term.show_fmt(precedence_range, f),
            VdBsqNonProductNumTerm::AtomInum(term) => term.show_fmt(precedence_range, f),
            VdBsqNonProductNumTerm::SumInum(term) => term.show_fmt(precedence_range, f),
        }
    }

    pub fn outer_precedence(&self) -> VdPrecedence {
        match self {
            VdBsqNonProductNumTerm::Rnum(term) => term.outer_precedence(),
            VdBsqNonProductNumTerm::AtomInum(term) => term.outer_precedence(),
            VdBsqNonProductNumTerm::SumInum(term) => VdPrecedence::ADD_SUB,
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
        f.write_str("NonSumInumTerm(`")?;
        self.show_fmt(VdPrecedenceRange::Any, f)?;
        f.write_str("`)")
    }
}

impl<'sess> VdBsqNonSumInumTerm<'sess> {
    pub fn show_fmt(
        self,
        precedence_range: VdPrecedenceRange,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            VdBsqNonSumInumTerm::Atom(term) => term.show_fmt(precedence_range, f),
            VdBsqNonSumInumTerm::Product(term) => term.show_fmt(precedence_range, f),
        }
    }

    pub fn outer_precedence(&self) -> VdPrecedence {
        match self {
            VdBsqNonSumInumTerm::Atom(term) => term.outer_precedence(),
            VdBsqNonSumInumTerm::Product(term) => term.outer_precedence(),
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
pub type VdBsqExponentialPowersRef<'a, 'sess> =
    &'a [(VdBsqNonProductNumTerm<'sess>, VdBsqNumTerm<'sess>)];
pub type VdBsqExponentialParts<'sess> = Vec<(VdBsqNonProductNumTerm<'sess>, VdBsqNumTerm<'sess>)>;

impl<'sess> std::fmt::Debug for VdBsqInumTermFld<'sess> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("InumTermFld(`")?;
        self.data().show_fmt(VdPrecedenceRange::Any, f)?;
        f.write_str("`)")
    }
}

impl<'sess> VdBsqInumTermData<'sess> {
    pub fn show_fmt(
        &self,
        precedence_range: VdPrecedenceRange,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            VdBsqInumTermData::Atom(term) => term.show_fmt(precedence_range, f),
            VdBsqInumTermData::Sum(term) => term.show_fmt(precedence_range, f),
            VdBsqInumTermData::Product(term) => term.show_fmt(precedence_range, f),
        }
    }
}
