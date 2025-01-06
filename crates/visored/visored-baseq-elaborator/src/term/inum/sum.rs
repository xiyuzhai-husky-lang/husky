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

    pub fn constant_term(&self) -> VdBsqRnumTerm {
        self.data().constant_term()
    }

    pub fn monomials(&self) -> &VdBsqInumMonomialCoefficients<'sess> {
        self.data().irrational_monomial_coefficients()
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

impl<'sess> VdBsqSumInumTerm<'sess> {
    pub fn show_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data().show_fmt(f)
    }
}

impl<'sess> VdBsqInumSumTermData<'sess> {
    pub fn show_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.constant_term.show_fmt(f)?;
        for (monomial, coefficient) in self.monomials.iter() {
            f.write_str(" + ")?;
            coefficient.show_fmt(f)?;
            monomial.show_fmt(f)?;
        }
        Ok(())
    }
}
