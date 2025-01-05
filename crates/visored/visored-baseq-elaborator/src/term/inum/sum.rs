use super::*;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct VdBsqSumInumTerm<'sess>(VdBsqInumTermFld<'sess>);

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct VdBsqInumSumTermData<'sess> {
    constant_term: VdBsqRnumTerm,
    monomials: VdBsqInumMonomialCoefficients<'sess>,
}

impl<'sess> From<VdBsqSumInumTerm<'sess>> for VdBsqNumTerm<'sess> {
    fn from(value: VdBsqSumInumTerm<'sess>) -> Self {
        VdBsqNumTerm::Inum(VdBsqInumTerm::Sum(value))
    }
}

impl<'sess> VdBsqSumInumTerm<'sess> {
    pub fn new(
        constant_term: VdBsqRnumTerm,
        monomials: VdBsqInumMonomialCoefficients<'sess>,
        db: &'sess FloaterDb,
    ) -> Self {
        Self(VdBsqInumTermFld::new(
            VdBsqInumTermData::Sum(VdBsqInumSumTermData {
                constant_term,
                monomials,
            }),
            db,
        ))
    }
}

impl<'sess> VdBsqSumInumTerm<'sess> {
    pub fn data(self) -> &'sess VdBsqInumSumTermData<'sess> {
        match self.0.data() {
            VdBsqInumTermData::Sum(data) => data,
            _ => unreachable!(),
        }
    }
}

impl<'sess> VdBsqInumSumTermData<'sess> {
    pub fn constant_term(&self) -> VdBsqRnumTerm {
        self.constant_term
    }

    pub fn irrational_monomial_coefficients(&self) -> &VdBsqInumMonomialCoefficients<'sess> {
        &self.monomials
    }
}

impl<'sess> VdBsqInumSumTermData<'sess> {
    pub fn show_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<'sess> VdBsqSumInumTerm<'sess> {
    pub fn show_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
