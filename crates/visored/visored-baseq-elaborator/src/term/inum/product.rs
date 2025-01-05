use super::*;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct VdBsqProductInumTermBase<'sess>(VdBsqInumTermFld<'sess>);

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct VdBsqProductInumTermBaseData<'sess> {
    exponentials: VdBsqExponentials<'sess>,
}

impl<'sess> VdBsqProductInumTermBaseData<'sess> {
    pub fn exponentials(&self) -> &[(VdBsqNonProductNumTerm<'sess>, VdBsqNumTerm<'sess>)] {
        &self.exponentials
    }
}

impl<'sess> VdBsqProductInumTermBase<'sess> {
    pub fn new(data: VdBsqProductInumTermBaseData<'sess>, db: &'sess FloaterDb) -> Self {
        VdBsqProductInumTermBase(VdBsqInumTermFld::new(VdBsqInumTermData::Product(data), db))
    }

    pub fn new_power(
        base: VdBsqNonProductNumTerm<'sess>,
        exponent: VdBsqNumTerm<'sess>,
        db: &'sess FloaterDb,
    ) -> Self {
        let exponentials = [(base, exponent)].into_iter().collect();
        Self::new(VdBsqProductInumTermBaseData { exponentials }, db)
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
    pub fn new_product(data: VdBsqProductInumTermBaseData<'sess>, db: &'sess FloaterDb) -> Self {
        VdBsqInumTerm::Product(todo!(), VdBsqProductInumTermBase::new(data, db))
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
