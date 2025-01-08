use super::*;
use crate::term::{
    atom::VdBsqAtomInumTerm,
    product::{VdBsqProductInumTermBase, VdBsqProductInumTermBaseData},
    sum::VdBsqSumInumTerm,
    VdBsqExponentialPowers, VdBsqInumMonomialCoefficients, VdBsqInumTerm, VdBsqLitNumTerm,
    VdBsqNonProductNumTerm, VdBsqNonSumInumTerm, VdBsqNumTerm,
};
use floated_sequential::db::FloaterDb;

pub struct VdBsqProductBuilder<'sess> {
    db: &'sess FloaterDb,
    /// Only for numbers representable efficiently by computers.
    /// For huge numbers like `2^100000`, we don't want to put it here.
    lit_num_coefficient: VdBsqLitNumTerm<'sess>,
    unpruned_exponentials: VdBsqExponentialPowers<'sess>,
}

impl<'sess> VdBsqProductBuilder<'sess> {
    pub fn new(db: &'sess FloaterDb) -> Self {
        Self {
            db,
            lit_num_coefficient: VdBsqLitNumTerm::ONE,
            unpruned_exponentials: VdBsqExponentialPowers::default(),
        }
    }

    pub fn new_from_num(num: VdBsqNumTerm<'sess>, db: &'sess FloaterDb) -> Self {
        match num {
            VdBsqNumTerm::Rnum(rnum) => Self::new_from_rnum(rnum, db),
            VdBsqNumTerm::Inum(inum) => Self::new_from_inum(inum, db),
        }
    }

    pub fn new_from_rnum(rnum: VdBsqLitNumTerm<'sess>, db: &'sess FloaterDb) -> Self {
        Self {
            db,
            lit_num_coefficient: rnum,
            unpruned_exponentials: VdBsqExponentialPowers::default(),
        }
    }

    pub fn new_from_inum(inum: VdBsqInumTerm<'sess>, db: &'sess FloaterDb) -> Self {
        match inum {
            VdBsqInumTerm::Atom(atom) => Self::new_from_atom(atom, db),
            VdBsqInumTerm::Sum(sum) => Self::new_from_sum(sum, db),
            VdBsqInumTerm::Product(rnum, product) => Self::new_from_product(rnum, product, db),
        }
    }

    pub fn new_from_atom(atom: VdBsqAtomInumTerm<'sess>, db: &'sess FloaterDb) -> Self {
        todo!()
    }

    pub fn new_from_sum(sum: VdBsqSumInumTerm<'sess>, db: &'sess FloaterDb) -> Self {
        todo!()
    }

    pub fn new_from_product(
        lit_num_coefficient: VdBsqLitNumTerm<'sess>,
        product: VdBsqProductInumTermBase<'sess>,
        db: &'sess FloaterDb,
    ) -> Self {
        Self {
            db,
            lit_num_coefficient: lit_num_coefficient,
            unpruned_exponentials: product.exponentials().clone(),
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

    pub fn mul_rnum(&mut self, rnum: VdBsqLitNumTerm<'sess>) {
        self.lit_num_coefficient.mul_assign(rnum, self.db);
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

    pub fn mul_product(
        &mut self,
        rnum: VdBsqLitNumTerm<'sess>,
        product: VdBsqProductInumTermBase<'sess>,
    ) {
        self.mul_rnum(rnum);
        for &(base, exponent) in product.exponentials() {
            self.mul_exponential(base, exponent);
        }
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

    pub fn div_num(&mut self, num: VdBsqNumTerm<'sess>) {
        match num {
            VdBsqNumTerm::Rnum(rnum) => self.div_lit_num(rnum),
            VdBsqNumTerm::Inum(inum) => self.div_inum(inum),
        }
    }

    pub fn div_lit_num(&mut self, lit_num: VdBsqLitNumTerm<'sess>) {
        self.lit_num_coefficient.div_assign(lit_num, self.db);
    }

    pub fn div_inum(&mut self, inum: VdBsqInumTerm<'sess>) {
        todo!()
    }

    pub fn finish(self) -> VdBsqNumTerm<'sess> {
        match self.lit_num_coefficient {
            VdBsqLitNumTerm::ZERO => VdBsqNumTerm::ZERO,
            lit_num_coefficient => {
                let exponentials: VdBsqExponentialPowers<'sess> =
                    self.unpruned_exponentials.into_iter().collect();
                if exponentials.is_empty() {
                    return VdBsqNumTerm::Rnum(self.lit_num_coefficient);
                }
                if lit_num_coefficient.is_one() && exponentials.len() == 1 {
                    let (base, exponent) = exponentials.data()[0];
                    if exponent.is_one_trivially() {
                        return base.into();
                    }
                }
                VdBsqNumTerm::new_product(self.lit_num_coefficient, exponentials, self.db)
            }
        }
    }
}
