use visored_opr::precedence::{VdPrecedence, VdPrecedenceRange};

use super::*;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqProductBase<'sess> {
    Atom(VdBsqAtomComnumTerm<'sess>),
    Sum(VdBsqSumComnumTerm<'sess>),
    NonTrivial(VdBsqNonTrivialProductBase<'sess>),
}

#[floated]
pub struct VdBsqNonTrivialProductBase<'sess> {
    #[return_ref]
    data: VdBsqProductComnumTermBaseData<'sess>,
}

impl<'sess> std::fmt::Debug for VdBsqNonTrivialProductBase<'sess> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VdBsqNonTrivialProductBase")
            .field("data", self.data())
            .finish()
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct VdBsqProductComnumTermBaseData<'sess> {
    exponentials: VdBsqExponentialPowers<'sess>,
}

impl<'sess> VdBsqProductComnumTermBaseData<'sess> {
    pub fn exponentials(&self) -> &VdBsqExponentialPowers<'sess> {
        &self.exponentials
    }
}

impl<'sess> VdBsqProductBase<'sess> {
    pub fn new(exponentials: VdBsqExponentialPowers<'sess>, db: &'sess FloaterDb) -> Self {
        #[cfg(debug_assertions)]
        {
            debug_assert!(exponentials.len() == 1);
            if exponentials.len() == 1 {
                let (base, exponent) = exponentials.data()[0];
                // debug_assert!(exponent.is_one_trivially());
                // todo!()
            }
        }
        todo!()
        // VdBsqProductBase(VdBsqComnumTermFld::new(
        //     VdBsqComnumTermData::Product(VdBsqProductComnumTermBaseData { exponentials }),
        //     db,
        // ))
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

impl<'sess> VdBsqProductBase<'sess> {
    pub fn data(&self) -> &'sess VdBsqProductComnumTermBaseData<'sess> {
        todo!()
        // match self.0.data() {
        //     VdBsqComnumTermData::Product(data) => data,
        //     _ => unreachable!(),
        // }
    }

    pub fn exponentials(&self) -> &'sess VdBsqExponentialPowers<'sess> {
        self.data().exponentials()
    }
}

impl<'sess> VdBsqComnumTerm<'sess> {
    pub fn new_product(
        litn_coefficient: VdBsqLitnumTerm<'sess>,
        exponentials: VdBsqExponentialPowers<'sess>,
        db: &'sess FloaterDb,
    ) -> Self {
        VdBsqComnumTerm::Product(litn_coefficient, VdBsqProductBase::new(exponentials, db))
    }

    pub fn new_power(
        base: VdBsqNonProductNumTerm<'sess>,
        exponent: VdBsqNumTerm<'sess>,
        db: &'sess FloaterDb,
    ) -> Self {
        match base {
            VdBsqNonProductNumTerm::Litnum(term) => todo!(),
            _ => match exponent {
                VdBsqNumTerm::ZERO => todo!(),
                VdBsqNumTerm::ONE => todo!(),
                VdBsqNumTerm::Comnum(VdBsqComnumTerm::Sum(term)) => todo!(),
                _ => (),
            },
        }
        VdBsqComnumTerm::Product(
            VdBsqLitnumTerm::ONE,
            VdBsqProductBase::new_power(base, exponent, db),
        )
    }
}

impl<'sess> VdBsqNumTerm<'sess> {
    pub fn new_product(
        litn_coefficient: VdBsqLitnumTerm<'sess>,
        exponentials: VdBsqExponentialPowers<'sess>,
        db: &'sess FloaterDb,
    ) -> Self {
        VdBsqNumTerm::Comnum(VdBsqComnumTerm::new_product(
            litn_coefficient,
            exponentials,
            db,
        ))
    }

    pub fn new_power(
        base: VdBsqNonProductNumTerm<'sess>,
        exponent: VdBsqNumTerm<'sess>,
        db: &'sess FloaterDb,
    ) -> Self {
        VdBsqNumTerm::Comnum(VdBsqComnumTerm::new_power(base, exponent, db))
    }
}

impl<'sess> VdBsqTerm<'sess> {
    pub fn new_power(
        base: VdBsqNonProductNumTerm<'sess>,
        exponent: VdBsqNumTerm<'sess>,
        db: &'sess FloaterDb,
    ) -> Self {
        VdBsqTerm::Comnum(VdBsqComnumTerm::new_power(base, exponent, db))
    }
}

impl<'sess> VdBsqProductComnumTermBaseData<'sess> {
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

impl<'sess> VdBsqProductBase<'sess> {
    pub fn show_fmt(
        &self,
        precedence_range: VdPrecedenceRange,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        self.data().show_fmt(precedence_range, f)
    }

    pub fn outer_precedence(&self) -> VdPrecedence {
        match self {
            VdBsqProductBase::Atom(term) => term.outer_precedence(),
            VdBsqProductBase::Sum(term) => term.outer_precedence(),
            VdBsqProductBase::NonTrivial(term) => term.outer_precedence(),
        }
    }
}

impl<'sess> VdBsqNonTrivialProductBase<'sess> {
    pub fn outer_precedence(&self) -> VdPrecedence {
        self.data().outer_precedence()
    }

    pub fn exponentials(&self) -> &'sess VdBsqExponentialPowers<'sess> {
        self.data().exponentials()
    }
}
