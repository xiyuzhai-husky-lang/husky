use super::*;
use crate::term::{
    atom::VdBsqAtomTerm,
    product::{VdBsqProductStem, VdBsqProductTerm},
    sum::VdBsqSumTerm,
    VdBsqComnumTerm, VdBsqExponentialPowers, VdBsqLitnumTerm, VdBsqMonomialCoefficients,
    VdBsqNonProductNumTerm, VdBsqNumTerm,
};
use floated_sequential::db::FloaterDb;

pub struct VdBsqProductBuilder<'sess> {
    db: &'sess FloaterDb,
    /// Only for numbers representable efficiently by computers.
    /// For huge numbers like `2^100000`, we don't want to put it here.
    litnum_factor: VdBsqLitnumTerm<'sess>,
    unpruned_exponentials: VdBsqExponentialPowers<'sess>,
}

impl<'sess> VdBsqProductBuilder<'sess> {
    pub fn new(db: &'sess FloaterDb) -> Self {
        Self {
            db,
            litnum_factor: VdBsqLitnumTerm::ONE,
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
            litnum_factor: litnum,
            unpruned_exponentials: VdBsqExponentialPowers::default(),
        }
    }

    pub fn new_from_comnum(comnum: VdBsqComnumTerm<'sess>, db: &'sess FloaterDb) -> Self {
        match comnum {
            VdBsqComnumTerm::Atom(atom) => Self::new_from_atom(atom, db),
            VdBsqComnumTerm::Sum(sum) => Self::new_from_sum(sum, db),
            VdBsqComnumTerm::Product(product) => Self::new_from_product(product, db),
        }
    }

    pub fn new_from_atom(atom: VdBsqAtomTerm<'sess>, db: &'sess FloaterDb) -> Self {
        todo!()
    }

    pub fn new_from_sum(sum: VdBsqSumTerm<'sess>, db: &'sess FloaterDb) -> Self {
        todo!()
    }

    pub fn new_from_product(product: VdBsqProductTerm<'sess>, db: &'sess FloaterDb) -> Self {
        Self {
            db,
            litnum_factor: product.litnum_factor(),
            unpruned_exponentials: match product.base() {
                VdBsqProductStem::Atom(atom) => [(atom.into(), 1.into())].into_iter().collect(),
                VdBsqProductStem::NonTrivial(non_trivial_product_base) => {
                    non_trivial_product_base.exponentials().clone()
                }
            },
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
        self.litnum_factor.mul_assign(litnum, self.db);
    }

    pub fn mul_comnum(&mut self, comnum: VdBsqComnumTerm<'sess>) {
        match comnum {
            VdBsqComnumTerm::Atom(atom) => self.mul_atom(atom),
            VdBsqComnumTerm::Sum(sum) => self.mul_sum(sum),
            VdBsqComnumTerm::Product(product) => self.mul_product(product),
        }
    }

    pub fn mul_atom(&mut self, atom: VdBsqAtomTerm<'sess>) {
        self.unpruned_exponentials
            .insert_or_update((atom.into(), VdBsqNumTerm::ONE), |(_, old_coeff)| {
                old_coeff.add_assign(VdBsqNumTerm::ONE, self.db)
            });
    }

    pub fn mul_sum(&mut self, sum: VdBsqSumTerm<'sess>) {
        self.unpruned_exponentials
            .insert_or_update((sum.into(), VdBsqNumTerm::ONE), |(_, old_coeff)| {
                old_coeff.add_assign(VdBsqNumTerm::ONE, self.db)
            });
    }

    pub fn mul_product(&mut self, product: VdBsqProductTerm<'sess>) {
        self.mul_litnum(product.litnum_factor());
        match product.base() {
            VdBsqProductStem::Atom(base) => self.mul_atom(base),
            VdBsqProductStem::NonTrivial(base) => {
                for &(base, exponent) in base.exponentials() {
                    self.mul_exponential(base, exponent);
                }
            }
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
        self.litnum_factor.div_assign(litn, self.db);
    }

    pub fn div_comnum(&mut self, comnum: VdBsqComnumTerm<'sess>) {
        todo!()
    }

    pub fn finish(self) -> VdBsqNumTerm<'sess> {
        match self.litnum_factor {
            VdBsqLitnumTerm::ZERO => VdBsqNumTerm::ZERO,
            litn_coefficient => {
                let exponentials: VdBsqExponentialPowers<'sess> =
                    self.unpruned_exponentials.into_iter().collect();
                if exponentials.is_empty() {
                    return VdBsqNumTerm::Litnum(self.litnum_factor);
                }
                if litn_coefficient.is_one() && exponentials.len() == 1 {
                    let (base, exponent) = exponentials.data()[0];
                    if exponent.is_one_trivially() {
                        return base.into();
                    }
                }
                VdBsqNumTerm::new_product(self.litnum_factor, exponentials, self.db)
            }
        }
    }
}
