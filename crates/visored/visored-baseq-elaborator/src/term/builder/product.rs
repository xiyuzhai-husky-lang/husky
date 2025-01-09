use super::*;
use crate::term::{
    atom::VdBsqAtomComnumTerm,
    product::{VdBsqProductComnumTermBase, VdBsqProductComnumTermBaseData},
    sum::VdBsqSumComnumTerm,
    VdBsqComnumTerm, VdBsqExponentialPowers, VdBsqLitnumTerm, VdBsqMonomialCoefficients,
    VdBsqNonProductNumTerm, VdBsqNonSumComnumTerm, VdBsqNumTerm,
};
use floated_sequential::db::FloaterDb;

pub struct VdBsqProductBuilder<'sess> {
    db: &'sess FloaterDb,
    /// Only for numbers representable efficiently by computers.
    /// For huge numbers like `2^100000`, we don't want to put it here.
    litn_coefficient: VdBsqLitnumTerm<'sess>,
    unpruned_exponentials: VdBsqExponentialPowers<'sess>,
}

impl<'sess> VdBsqProductBuilder<'sess> {
    pub fn new(db: &'sess FloaterDb) -> Self {
        Self {
            db,
            litn_coefficient: VdBsqLitnumTerm::ONE,
            unpruned_exponentials: VdBsqExponentialPowers::default(),
        }
    }

    pub fn new_from_num(num: VdBsqNumTerm<'sess>, db: &'sess FloaterDb) -> Self {
        match num {
            VdBsqNumTerm::Litnum(litnum) => Self::new_from_litnum(litnum, db),
            VdBsqNumTerm::Comnum(comnum) => Self::new_from_comnum(comnum, db),
        }
    }

    pub fn new_from_litnum(litnum: VdBsqLitnumTerm<'sess>, db: &'sess FloaterDb) -> Self {
        Self {
            db,
            litn_coefficient: litnum,
            unpruned_exponentials: VdBsqExponentialPowers::default(),
        }
    }

    pub fn new_from_comnum(comnum: VdBsqComnumTerm<'sess>, db: &'sess FloaterDb) -> Self {
        match comnum {
            VdBsqComnumTerm::Atom(atom) => Self::new_from_atom(atom, db),
            VdBsqComnumTerm::Sum(sum) => Self::new_from_sum(sum, db),
            VdBsqComnumTerm::Product(litnum, product) => {
                Self::new_from_product(litnum, product, db)
            }
        }
    }

    pub fn new_from_atom(atom: VdBsqAtomComnumTerm<'sess>, db: &'sess FloaterDb) -> Self {
        todo!()
    }

    pub fn new_from_sum(sum: VdBsqSumComnumTerm<'sess>, db: &'sess FloaterDb) -> Self {
        todo!()
    }

    pub fn new_from_product(
        litn_coefficient: VdBsqLitnumTerm<'sess>,
        product: VdBsqProductComnumTermBase<'sess>,
        db: &'sess FloaterDb,
    ) -> Self {
        Self {
            db,
            litn_coefficient: litn_coefficient,
            unpruned_exponentials: product.exponentials().clone(),
        }
    }
}

impl<'sess> VdBsqProductBuilder<'sess> {
    pub fn mul_num(&mut self, num: VdBsqNumTerm<'sess>) {
        match num {
            VdBsqNumTerm::Litnum(litnum) => self.mul_litnum(litnum),
            VdBsqNumTerm::Comnum(comnum) => self.mul_comnum(comnum),
        }
    }

    pub fn mul_litnum(&mut self, litnum: VdBsqLitnumTerm<'sess>) {
        self.litn_coefficient.mul_assign(litnum, self.db);
    }

    pub fn mul_comnum(&mut self, comnum: VdBsqComnumTerm<'sess>) {
        match comnum {
            VdBsqComnumTerm::Atom(atom) => self.mul_atom(atom),
            VdBsqComnumTerm::Sum(sum) => self.mul_sum(sum),
            VdBsqComnumTerm::Product(litnum, product) => self.mul_product(litnum, product),
        }
    }

    pub fn mul_atom(&mut self, atom: VdBsqAtomComnumTerm<'sess>) {
        self.unpruned_exponentials
            .insert_or_update((atom.into(), VdBsqNumTerm::ONE), |(_, old_coeff)| {
                old_coeff.add_assign(VdBsqNumTerm::ONE, self.db)
            });
    }

    pub fn mul_sum(&mut self, sum: VdBsqSumComnumTerm<'sess>) {
        self.unpruned_exponentials
            .insert_or_update((sum.into(), VdBsqNumTerm::ONE), |(_, old_coeff)| {
                old_coeff.add_assign(VdBsqNumTerm::ONE, self.db)
            });
    }

    pub fn mul_product(
        &mut self,
        litnum: VdBsqLitnumTerm<'sess>,
        product: VdBsqProductComnumTermBase<'sess>,
    ) {
        self.mul_litnum(litnum);
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
            VdBsqNumTerm::Litnum(litnum) => self.div_litnum(litnum),
            VdBsqNumTerm::Comnum(comnum) => self.div_comnum(comnum),
        }
    }

    pub fn div_litnum(&mut self, litn: VdBsqLitnumTerm<'sess>) {
        self.litn_coefficient.div_assign(litn, self.db);
    }

    pub fn div_comnum(&mut self, comnum: VdBsqComnumTerm<'sess>) {
        todo!()
    }

    pub fn finish(self) -> VdBsqNumTerm<'sess> {
        match self.litn_coefficient {
            VdBsqLitnumTerm::ZERO => VdBsqNumTerm::ZERO,
            litn_coefficient => {
                let exponentials: VdBsqExponentialPowers<'sess> =
                    self.unpruned_exponentials.into_iter().collect();
                if exponentials.is_empty() {
                    return VdBsqNumTerm::Litnum(self.litn_coefficient);
                }
                if litn_coefficient.is_one() && exponentials.len() == 1 {
                    let (base, exponent) = exponentials.data()[0];
                    if exponent.is_one_trivially() {
                        return base.into();
                    }
                }
                VdBsqNumTerm::new_product(self.litn_coefficient, exponentials, self.db)
            }
        }
    }
}
