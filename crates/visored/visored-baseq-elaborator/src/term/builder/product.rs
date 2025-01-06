use super::*;
use crate::term::{
    atom::VdBsqAtomInumTerm,
    product::{VdBsqProductInumTermBase, VdBsqProductInumTermBaseData},
    sum::VdBsqSumInumTerm,
    VdBsqExponentialPowers, VdBsqInumMonomialCoefficients, VdBsqInumTerm, VdBsqNonSumInumTerm,
    VdBsqNumTerm, VdBsqRnumTerm,
};
use floated_sequential::db::FloaterDb;

pub struct VdBsqProductBuilder<'sess> {
    db: &'sess FloaterDb,
    /// Only for numbers representable efficiently by computers.
    /// For huge numbers like `2^100000`, we don't want to put it here.
    rnum_coefficient: VdBsqRnumTerm,
    unpruned_exponentials: VdBsqExponentialPowers<'sess>,
}

impl<'sess> VdBsqProductBuilder<'sess> {
    pub fn new(db: &'sess FloaterDb) -> Self {
        Self {
            db,
            rnum_coefficient: VdBsqRnumTerm::ONE,
            unpruned_exponentials: VdBsqExponentialPowers::default(),
        }
    }
}

impl<'sess> VdBsqProductBuilder<'sess> {
    pub fn mul_num(&mut self, num: VdBsqNumTerm<'sess>) {
        match num {
            VdBsqNumTerm::Rnum(rnum) => self.mul_rnum(rnum),
            VdBsqNumTerm::Inum(inum) => self.mul_inum(inum),
        }
    }

    pub fn mul_rnum(&mut self, rnum: VdBsqRnumTerm) {
        self.rnum_coefficient *= rnum;
    }

    pub fn mul_inum(&mut self, inum: VdBsqInumTerm<'sess>) {
        match inum {
            VdBsqInumTerm::Atom(atom) => self.mul_atom(atom),
            VdBsqInumTerm::Sum(sum) => self.mul_sum(sum),
            VdBsqInumTerm::Product(rnum, product) => self.mul_product(rnum, product),
        }
    }

    pub fn mul_atom(&mut self, atom: VdBsqAtomInumTerm<'sess>) {
        self.unpruned_exponentials
            .insert_or_update((atom.into(), VdBsqNumTerm::ONE), |(_, old_coeff)| {
                old_coeff.add_assign(VdBsqNumTerm::ONE, self.db)
            });
    }

    pub fn mul_sum(&mut self, sum: VdBsqSumInumTerm<'sess>) {
        todo!()
    }

    pub fn mul_product(&mut self, rnum: VdBsqRnumTerm, product: VdBsqProductInumTermBase<'sess>) {
        todo!()
    }

    pub fn finish(self) -> VdBsqNumTerm<'sess> {
        match self.rnum_coefficient {
            VdBsqRnumTerm::ZERO => VdBsqNumTerm::ZERO,
            _ => {
                let exponentials = self.unpruned_exponentials.into_iter().collect();
                VdBsqNumTerm::new_product(self.rnum_coefficient, exponentials, self.db)
            }
        }
    }
}
