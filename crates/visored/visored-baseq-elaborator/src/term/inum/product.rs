use super::*;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct VdBsqProductInumTerm<'sess>(VdBsqInumTermFld<'sess>);

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct VdBsqProductInumTermData<'sess> {
    rational: VdBsqRnumTerm,
    irrational_atom_exponentials: VdBsqInumAtomExponentials<'sess>,
}

impl<'sess> VdBsqProductInumTermData<'sess> {
    pub fn rational(&self) -> VdBsqRnumTerm {
        self.rational
    }

    pub fn irrational_atom_exponentials(
        &self,
    ) -> &[(VdBsqNonProductNumTerm<'sess>, VdBsqNumTerm<'sess>)] {
        &self.irrational_atom_exponentials
    }
}

impl<'sess> VdBsqProductInumTerm<'sess> {
    pub fn new(data: VdBsqProductInumTermData<'sess>, db: &'sess FloaterDb) -> Self {
        VdBsqProductInumTerm(VdBsqInumTermFld::new(VdBsqInumTermData::Product(data), db))
    }

    pub fn new_power(
        base: VdBsqNonProductNumTerm<'sess>,
        exponent: VdBsqNumTerm<'sess>,
        db: &'sess FloaterDb,
    ) -> Self {
        let rational = VdBsqRnumTerm::ONE;
        let irrational_atom_exponentials = [(base, exponent)].into_iter().collect();
        Self::new(
            VdBsqProductInumTermData {
                rational,
                irrational_atom_exponentials,
            },
            db,
        )
    }
}

impl<'sess> VdBsqProductInumTerm<'sess> {
    pub fn data(&self) -> &'sess VdBsqProductInumTermData<'sess> {
        match self.0.data() {
            VdBsqInumTermData::Product(data) => data,
            _ => unreachable!(),
        }
    }
}

impl<'sess> VdBsqInumTerm<'sess> {
    pub fn new_product(data: VdBsqProductInumTermData<'sess>, db: &'sess FloaterDb) -> Self {
        VdBsqInumTerm::Product(VdBsqProductInumTerm::new(data, db))
    }

    pub fn new_power(
        base: VdBsqNonProductNumTerm<'sess>,
        exponent: VdBsqNumTerm<'sess>,
        db: &'sess FloaterDb,
    ) -> Self {
        VdBsqInumTerm::Product(VdBsqProductInumTerm::new_power(base, exponent, db))
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
