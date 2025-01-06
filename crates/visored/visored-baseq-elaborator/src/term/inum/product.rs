use visored_opr::precedence::{VdPrecedence, VdPrecedenceRange};

use super::*;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct VdBsqProductInumTermBase<'sess>(VdBsqInumTermFld<'sess>);

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct VdBsqProductInumTermBaseData<'sess> {
    exponentials: VdBsqExponentialPowers<'sess>,
}

impl<'sess> VdBsqProductInumTermBaseData<'sess> {
    pub fn exponentials(&self) -> &[(VdBsqNonProductNumTerm<'sess>, VdBsqNumTerm<'sess>)] {
        &self.exponentials
    }
}

impl<'sess> VdBsqProductInumTermBase<'sess> {
    pub fn new(exponentials: VdBsqExponentialPowers<'sess>, db: &'sess FloaterDb) -> Self {
        VdBsqProductInumTermBase(VdBsqInumTermFld::new(
            VdBsqInumTermData::Product(VdBsqProductInumTermBaseData { exponentials }),
            db,
        ))
    }

    pub fn from_parts(
        exponentials: VdBsqExponentialParts<'sess>,
        db: &'sess FloaterDb,
    ) -> VdBsqNumTerm<'sess> {
        let mut builder = VdBsqProductBuilder::new(db);
        for (base, exponent) in exponentials {
            builder.mul_exponential(base, exponent);
        }
        builder.finish()
    }

    pub fn new_power(
        base: VdBsqNonProductNumTerm<'sess>,
        exponent: VdBsqNumTerm<'sess>,
        db: &'sess FloaterDb,
    ) -> Self {
        let exponentials = [(base, exponent)].into_iter().collect();
        Self::new(exponentials, db)
    }
}

impl<'sess> VdBsqProductInumTermBase<'sess> {
    pub fn data(&self) -> &'sess VdBsqProductInumTermBaseData<'sess> {
        match self.0.data() {
            VdBsqInumTermData::Product(data) => data,
            _ => unreachable!(),
        }
    }

    pub fn exponentials(&self) -> &'sess [(VdBsqNonProductNumTerm<'sess>, VdBsqNumTerm<'sess>)] {
        self.data().exponentials()
    }

    pub fn outer_precedence(&self) -> VdPrecedence {
        self.data().outer_precedence()
    }
}

impl<'sess> VdBsqInumTerm<'sess> {
    pub fn new_product(
        rnum_coefficient: VdBsqRnumTerm,
        exponentials: VdBsqExponentialPowers<'sess>,
        db: &'sess FloaterDb,
    ) -> Self {
        VdBsqInumTerm::Product(
            rnum_coefficient,
            VdBsqProductInumTermBase::new(exponentials, db),
        )
    }

    pub fn new_power(
        base: VdBsqNonProductNumTerm<'sess>,
        exponent: VdBsqNumTerm<'sess>,
        db: &'sess FloaterDb,
    ) -> Self {
        match base {
            VdBsqNonProductNumTerm::Rnum(term) => todo!(),
            _ => match exponent {
                VdBsqNumTerm::ZERO => todo!(),
                VdBsqNumTerm::ONE => todo!(),
                VdBsqNumTerm::Inum(VdBsqInumTerm::Sum(term)) => todo!(),
                _ => (),
            },
        }
        VdBsqInumTerm::Product(
            VdBsqRnumTerm::ONE,
            VdBsqProductInumTermBase::new_power(base, exponent, db),
        )
    }
}

impl<'sess> VdBsqNumTerm<'sess> {
    pub fn new_product(
        rnum_coefficient: VdBsqRnumTerm,
        exponentials: VdBsqExponentialPowers<'sess>,
        db: &'sess FloaterDb,
    ) -> Self {
        VdBsqNumTerm::Inum(VdBsqInumTerm::new_product(
            rnum_coefficient,
            exponentials,
            db,
        ))
    }

    pub fn new_power(
        base: VdBsqNonProductNumTerm<'sess>,
        exponent: VdBsqNumTerm<'sess>,
        db: &'sess FloaterDb,
    ) -> Self {
        VdBsqNumTerm::Inum(VdBsqInumTerm::new_power(base, exponent, db))
    }
}

impl<'sess> VdBsqTerm<'sess> {
    pub fn new_power(
        base: VdBsqNonProductNumTerm<'sess>,
        exponent: VdBsqNumTerm<'sess>,
        db: &'sess FloaterDb,
    ) -> Self {
        VdBsqTerm::Inum(VdBsqInumTerm::new_power(base, exponent, db))
    }
}

impl<'sess> VdBsqProductInumTermBaseData<'sess> {
    pub fn show_fmt(
        &self,
        precedence_range: VdPrecedenceRange,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let outer_precedence = if self.exponentials.len() == 1 {
            let (base, exponent) = self.exponentials.data()[0];
            if exponent.is_one_trivially() {
                let base_outer_precedence = base.outer_precedence();
                if VdPrecedenceRange::MUL_DIV_LEFT.contains(base_outer_precedence) {
                    base_outer_precedence
                } else {
                    VdPrecedence::ATOM
                }
            } else {
                VdPrecedence::MUL_DIV
            }
        } else {
            VdPrecedence::MUL_DIV
        };
        if precedence_range.contains(outer_precedence) {
            self.show_fmt_inner(f)
        } else {
            f.write_str("(")?;
            self.show_fmt_inner(f)?;
            f.write_str(")")
        }
    }

    pub fn outer_precedence(&self) -> VdPrecedence {
        todo!()
        // VdPrecedence::MUL_DIV
    }

    fn show_fmt_inner(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &(base, exponent) in self.exponentials() {
            match exponent {
                VdBsqNumTerm::ZERO => unreachable!(),
                VdBsqNumTerm::ONE => base.show_fmt(VdPrecedenceRange::MUL_DIV_LEFT, f)?,
                _ => {
                    base.show_fmt(VdPrecedenceRange::ATOM, f)?;
                    f.write_str("^")?;
                    exponent.show_fmt(VdPrecedenceRange::ATOM, f)?;
                    f.write_str("")?
                }
            }
        }
        Ok(())
    }
}

impl<'sess> VdBsqProductInumTermBase<'sess> {
    pub fn show_fmt(
        &self,
        precedence_range: VdPrecedenceRange,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        self.data().show_fmt(precedence_range, f)
    }
}
