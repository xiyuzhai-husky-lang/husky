use super::*;
use crate::term::sum::VdBsqSumInumTerm;
use smallvec::*;

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
        let (lhs_constant_term, lhs_irrational_monomial_coefficients) = self.sum_decomposition();
        let (rhs_constant_term, rhs_irrational_monomial_coefficients) = rhs.sum_decomposition();
        todo!()
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
