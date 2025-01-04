use super::*;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct VdBsqInumSumTerm<'sess>(VdBsqInumTermFld<'sess>);

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct VdBsqInumSumTermData<'sess> {
    constant_term: VdBsqRnumTerm,
    irrational_monomial_coefficients: VdBsqInumMonomialCoefficients<'sess>,
}

impl<'sess> VdBsqInumSumTermData<'sess> {
    pub fn constant_term(&self) -> VdBsqRnumTerm {
        self.constant_term
    }

    pub fn irrational_monomial_coefficients(&self) -> &VdBsqInumMonomialCoefficients<'sess> {
        &self.irrational_monomial_coefficients
    }
}
