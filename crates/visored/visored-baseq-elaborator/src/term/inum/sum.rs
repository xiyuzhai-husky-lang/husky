use visored_opr::precedence::{VdPrecedence, VdPrecedenceRange};

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
    pub fn show_fmt(
        &self,
        precedence_range: VdPrecedenceRange,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        self.data().show_fmt(precedence_range, f)
    }
}

impl<'sess> VdBsqInumSumTermData<'sess> {
    pub fn show_fmt(
        &self,
        precedence_range: VdPrecedenceRange,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let outer_precedence = self.outer_precedence();
        if precedence_range.contains(outer_precedence) {
            self.show_fmt_inner(f)
        } else {
            f.write_str("(")?;
            self.show_fmt_inner(f)?;
            f.write_str(")")
        }
    }

    pub fn outer_precedence(&self) -> VdPrecedence {
        let num_of_summands = if self.constant_term.is_zero() {
            self.monomials.len()
        } else {
            self.monomials.len() + 1
        };
        if num_of_summands == 1 {
            let (monomial, coeff) = self.monomials.data()[0];
            match coeff {
                VdBsqRnumTerm::ONE => monomial.outer_precedence(),
                _ => VdPrecedence::MUL_DIV,
            }
        } else {
            VdPrecedence::REDUCE_PREFIX
        }
    }

    fn show_fmt_inner(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.constant_term.is_zero() {
            self.constant_term
                .show_fmt(VdPrecedenceRange::ADD_SUB_LEFT, f)?;
            f.write_str(" + ")?;
        }
        for (i, (monomial, coefficient)) in self.monomials.iter().enumerate() {
            if i > 0 {
                f.write_str(" + ")?;
            }
            if !coefficient.is_one() {
                coefficient.show_fmt(VdPrecedenceRange::MUL_DIV_RIGHT, f)?;
                match monomial {
                    VdBsqNonSumInumTerm::Atom(term) => (),
                    VdBsqNonSumInumTerm::Product(base) => match base.exponentials()[0].0 {
                        VdBsqNonProductNumTerm::Rnum(_) => f.write_str(" × ")?,
                        VdBsqNonProductNumTerm::AtomInum(_)
                        | VdBsqNonProductNumTerm::SumInum(_) => (),
                    },
                }
            }
            monomial.show_fmt(VdPrecedenceRange::MUL_DIV_RIGHT, f)?;
        }
        Ok(())
    }
}
