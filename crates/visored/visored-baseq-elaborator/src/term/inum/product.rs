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
    pub fn show_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (base, exponent) in self.exponentials() {
            base.show_fmt(f)?;
            f.write_str("^(")?;
            exponent.show_fmt(f)?;
            f.write_str(")")?;
        }
        Ok(())
    }
}

impl<'sess> VdBsqProductInumTermBase<'sess> {
    pub fn show_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data().show_fmt(f)
    }
}
