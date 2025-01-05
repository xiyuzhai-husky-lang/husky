use super::*;
use crate::term::sum::VdBsqSumInumTerm;
use builder::sum::VdBsqSumBuilder;
use smallvec::*;

#[enum_class::from_variants]
#[derive(Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqNumTerm<'sess> {
    Rnum(VdBsqRnumTerm),
    Inum(VdBsqInumTerm<'sess>),
}

impl<'sess> From<VdBsqNumTerm<'sess>> for VdBsqTerm<'sess> {
    fn from(term: VdBsqNumTerm<'sess>) -> Self {
        match term {
            VdBsqNumTerm::Rnum(term) => VdBsqTerm::Rnum(term),
            VdBsqNumTerm::Inum(term) => VdBsqTerm::Inum(term),
        }
    }
}

impl<'sess> VdBsqNumTerm<'sess> {
    pub const ZERO: Self = VdBsqNumTerm::Rnum(VdBsqRnumTerm::ZERO);
    pub const ONE: Self = VdBsqNumTerm::Rnum(VdBsqRnumTerm::ONE);
}

impl<'sess> VdBsqNumTerm<'sess> {
    pub fn is_zero_trivially(&self) -> bool {
        match self {
            VdBsqNumTerm::Rnum(term) => term.is_zero(),
            VdBsqNumTerm::Inum(term) => false,
        }
    }

    pub fn eqs_i128_trivially(&self, rhs: i128) -> bool {
        match self {
            VdBsqNumTerm::Rnum(term) => term.eqs_i128(rhs),
            VdBsqNumTerm::Inum(term) => false,
        }
    }

    pub fn sub(self, rhs: VdBsqNumTerm<'sess>, db: &'sess FloaterDb) -> VdBsqNumTerm<'sess> {
        if rhs.is_zero_trivially() {
            return self;
        }
        let mut builder = VdBsqSumBuilder::new(db);
        builder.add_num_term(self);
        builder.sub_num(rhs);
        builder.finish()
    }

    pub fn sum_decomposition(self) -> (VdBsqRnumTerm, VdBsqInumMonomialCoefficients<'sess>) {
        match self {
            VdBsqNumTerm::Rnum(term) => (term, VdBsqInumMonomialCoefficients::default()),
            VdBsqNumTerm::Inum(term) => match term {
                VdBsqInumTerm::Atom(term) => (
                    VdBsqRnumTerm::ZERO,
                    [(term.into(), VdBsqRnumTerm::ONE)].into_iter().collect(),
                ),
                VdBsqInumTerm::Sum(term) => (
                    term.data().constant_term(),
                    term.data().irrational_monomial_coefficients().clone(),
                ),
                VdBsqInumTerm::Product(rnum, term) => (
                    VdBsqRnumTerm::ZERO,
                    [(VdBsqNonSumInumTerm::Product(rnum, term), VdBsqRnumTerm::ONE)]
                        .into_iter()
                        .collect(),
                ),
            },
        }
    }
}
