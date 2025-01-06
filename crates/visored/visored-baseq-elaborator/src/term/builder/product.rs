use super::*;
use crate::term::{
    atom::VdBsqAtomInumTerm,
    product::{VdBsqProductInumTermBase, VdBsqProductInumTermBaseData},
    sum::VdBsqSumInumTerm,
    VdBsqExponentialPowers, VdBsqInumMonomialCoefficients, VdBsqInumTerm, VdBsqNonProductNumTerm,
    VdBsqNonSumInumTerm, VdBsqNumTerm, VdBsqRnumTerm,
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
        self.rnum_coefficient.mul_assign(rnum, self.db);
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
        self.unpruned_exponentials
            .insert_or_update((sum.into(), VdBsqNumTerm::ONE), |(_, old_coeff)| {
                old_coeff.add_assign(VdBsqNumTerm::ONE, self.db)
            });
    }

    pub fn mul_product(&mut self, rnum: VdBsqRnumTerm, product: VdBsqProductInumTermBase<'sess>) {
        todo!()
    }

    pub fn mul_exponential(
        &mut self,
        base: VdBsqNonProductNumTerm<'sess>,
        exponent: VdBsqNumTerm<'sess>,
    ) {
        self.unpruned_exponentials
            .insert_or_update((base, exponent), |(_, old_coeff)| {
                old_coeff.add_assign(exponent, self.db)
            });
    }

    pub fn finish(self) -> VdBsqNumTerm<'sess> {
        match self.rnum_coefficient {
            VdBsqRnumTerm::ZERO => VdBsqNumTerm::ZERO,
            rnum_coefficient => {
                let exponentials: VdBsqExponentialPowers<'sess> =
                    self.unpruned_exponentials.into_iter().collect();
                if exponentials.is_empty() {
                    return VdBsqNumTerm::Rnum(self.rnum_coefficient);
                }
                if rnum_coefficient.is_one() && exponentials.len() == 1 {
                    let (base, exponent) = exponentials.data()[0];
                    if exponent.is_one_trivially() {
                        return base.into();
                    }
                }
                VdBsqNumTerm::new_product(self.rnum_coefficient, exponentials, self.db)
            }
        }
    }
}
