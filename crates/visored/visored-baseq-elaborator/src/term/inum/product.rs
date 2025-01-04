use super::*;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct VdBsqInumProductTerm<'sess>(VdBsqInumTermFld<'sess>);

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct VdBsqInumProductTermData<'sess> {
    rational: VdBsqRnumTerm,
    irrational_atom_exponentials: VdBsqInumAtomExponentials<'sess>,
}

impl<'sess> VdBsqInumProductTermData<'sess> {
    pub fn rational(&self) -> VdBsqRnumTerm {
        self.rational
    }

    pub fn irrational_atom_exponentials(
        &self,
    ) -> &[(VdBsqNonProductNumTerm<'sess>, VdBsqNumTerm<'sess>)] {
        &self.irrational_atom_exponentials
    }
}

impl<'sess> VdBsqInumProductTerm<'sess> {
    pub fn new(data: VdBsqInumProductTermData<'sess>, db: &'sess FloaterDb) -> Self {
        VdBsqInumProductTerm(VdBsqInumTermFld::new(VdBsqInumTermData::Product(data), db))
    }

    pub fn new_power(
        base: VdBsqNumTerm<'sess>,
        exponent: VdBsqNumTerm<'sess>,
        db: &'sess FloaterDb,
    ) -> Self {
        let rational = VdBsqRnumTerm::ONE;
        let irrational_atom_exponentials = match base {
            VdBsqNumTerm::Rnum(term) => todo!(),
            VdBsqNumTerm::Inum(term) => todo!(),
        };
        Self::new(
            VdBsqInumProductTermData {
                rational,
                irrational_atom_exponentials,
            },
            db,
        )
    }
}

impl<'sess> VdBsqInumProductTerm<'sess> {
    pub fn data(&self) -> &'sess VdBsqInumProductTermData<'sess> {
        match self.0.data() {
            VdBsqInumTermData::Product(data) => data,
            _ => unreachable!(),
        }
    }
}

impl<'sess> VdBsqInumTerm<'sess> {
    pub fn new_product(data: VdBsqInumProductTermData<'sess>, db: &'sess FloaterDb) -> Self {
        VdBsqInumTerm::Product(VdBsqInumProductTerm::new(data, db))
    }

    pub fn new_power(
        base: VdBsqNumTerm<'sess>,
        exponent: VdBsqNumTerm<'sess>,
        db: &'sess FloaterDb,
    ) -> Self {
        VdBsqInumTerm::Product(VdBsqInumProductTerm::new_power(base, exponent, db))
    }
}

impl<'sess> VdBsqNumTerm<'sess> {
    pub fn new_power(
        base: VdBsqNumTerm<'sess>,
        exponent: VdBsqNumTerm<'sess>,
        db: &'sess FloaterDb,
    ) -> Self {
        VdBsqNumTerm::Inum(VdBsqInumTerm::new_power(base, exponent, db))
    }
}

impl<'sess> VdBsqTerm<'sess> {
    pub fn new_power(
        base: VdBsqNumTerm<'sess>,
        exponent: VdBsqNumTerm<'sess>,
        db: &'sess FloaterDb,
    ) -> Self {
        VdBsqTerm::Inum(VdBsqInumTerm::new_power(base, exponent, db))
    }
}
