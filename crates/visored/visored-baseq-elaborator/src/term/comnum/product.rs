use super::*;
use visored_opr::precedence::{VdPrecedence, VdPrecedenceRange};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct VdBsqProductTerm<'sess> {
    litnum_factor: VdBsqLitnumTerm<'sess>,
    base: VdBsqProductBase<'sess>,
}

impl<'sess> VdBsqProductTerm<'sess> {
    pub fn new(
        litnum_factor: impl Into<VdBsqLitnumTerm<'sess>>,
        base: impl Into<VdBsqProductBase<'sess>>,
    ) -> Self {
        Self {
            litnum_factor: litnum_factor.into(),
            base: base.into(),
        }
    }

    pub fn new2(
        litnum_factor: VdBsqLitnumTerm<'sess>,
        exponentials: VdBsqExponentialPowers<'sess>,
        db: &'sess FloaterDb,
    ) -> Self {
        let base = VdBsqProductBase::new(exponentials, db);
        Self {
            litnum_factor,
            base,
        }
    }

    pub fn with_litnum_factor(self, litnum_factor: VdBsqLitnumTerm<'sess>) -> Self {
        Self {
            litnum_factor,
            base: self.base,
        }
    }

    pub fn with_litnum_factor_update(
        self,
        f: impl FnOnce(VdBsqLitnumTerm<'sess>) -> VdBsqLitnumTerm<'sess>,
    ) -> Self {
        Self {
            litnum_factor: f(self.litnum_factor),
            base: self.base,
        }
    }
}

impl<'sess> VdBsqProductTerm<'sess> {
    pub fn litnum_factor(self) -> VdBsqLitnumTerm<'sess> {
        self.litnum_factor
    }

    pub fn base(&self) -> VdBsqProductBase<'sess> {
        self.base
    }

    pub fn exponentials(&self) -> &VdBsqExponentialPowers<'sess> {
        self.base.exponentials()
    }
}

impl<'sess> From<VdBsqProductBase<'sess>> for VdBsqProductTerm<'sess> {
    fn from(base: VdBsqProductBase<'sess>) -> Self {
        Self {
            litnum_factor: VdBsqLitnumTerm::ONE,
            base,
        }
    }
}

impl<'sess> From<VdBsqProductBase<'sess>> for VdBsqComnumTerm<'sess> {
    fn from(base: VdBsqProductBase<'sess>) -> Self {
        Self::Product(base.into())
    }
}

impl<'sess> VdBsqProductTerm<'sess> {
    pub fn show_fmt(
        self,
        precedence_range: VdPrecedenceRange,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let Self {
            litnum_factor: factor,
            base,
        } = self;
        debug_assert!(!factor.is_zero());
        if factor.is_one() {
            debug_assert!(!base.exponentials().is_empty());
            base.show_fmt(precedence_range, f)
        } else {
            fn show_product_fmt_inner<'sess>(
                litnum: VdBsqLitnumTerm<'sess>,
                term: VdBsqProductBase<'sess>,
                f: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                litnum.show_fmt(VdPrecedenceRange::MUL_DIV_LEFT, f)?;
                match term.exponentials().data()[0].0 {
                    VdBsqNonProductNumTerm::Litnum(_) => f.write_str(" × ")?,
                    VdBsqNonProductNumTerm::AtomComnum(_)
                    | VdBsqNonProductNumTerm::SumComnum(_) => (),
                }
                term.show_fmt(VdPrecedenceRange::MUL_DIV_RIGHT, f)
            }

            if precedence_range.contains(VdPrecedence::MUL_DIV) {
                show_product_fmt_inner(factor, base, f)
            } else {
                f.write_str("(")?;
                show_product_fmt_inner(factor, base, f)?;
                f.write_str(")")
            }
        }
    }
}

#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqProductBase<'sess> {
    Atom(VdBsqAtomTerm<'sess>),
    Sum(VdBsqSumTerm<'sess>),
    NonTrivial(VdBsqNonTrivialProductBase<'sess>),
}

#[floated]
pub struct VdBsqNonTrivialProductBase<'sess> {
    #[return_ref]
    data: VdBsqProductBaseData<'sess>,
}

impl<'sess> std::fmt::Debug for VdBsqNonTrivialProductBase<'sess> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VdBsqNonTrivialProductBase")
            .field("data", self.data())
            .finish()
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct VdBsqProductBaseData<'sess> {
    exponentials: VdBsqExponentialPowers<'sess>,
}

impl<'sess> VdBsqProductBaseData<'sess> {
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
    pub fn data(&self) -> &'sess VdBsqProductBaseData<'sess> {
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
        factor: VdBsqLitnumTerm<'sess>,
        exponentials: VdBsqExponentialPowers<'sess>,
        db: &'sess FloaterDb,
    ) -> Self {
        VdBsqProductTerm::new2(factor, exponentials, db).into()
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
        VdBsqProductBase::new_power(base, exponent, db).into()
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

impl<'sess> VdBsqProductBaseData<'sess> {
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
