use floated_sequential::db::FloaterDb;

use super::*;
use crate::term::{
    atom::VdBsqAtomInumTerm, product::VdBsqProductInumTermBase, sum::VdBsqSumInumTerm,
    VdBsqInumMonomialCoefficients, VdBsqInumTerm, VdBsqNonSumInumTerm, VdBsqNumTerm, VdBsqRnumTerm,
};

pub struct VdBsqSumBuilder<'sess> {
    db: &'sess FloaterDb,
    /// Only for numbers representable efficiently by computers.
    /// For huge numbers like `2^100000`, we don't want to put it here.
    constant_rnum: VdBsqRnumTerm,
    unpruned_monomials: VdBsqInumMonomialCoefficients<'sess>,
}
impl<'sess> VdBsqSumBuilder<'sess> {
    pub fn new(db: &'sess FloaterDb) -> Self {
        Self {
            db,
            constant_rnum: VdBsqRnumTerm::ZERO,
            unpruned_monomials: VdBsqInumMonomialCoefficients::default(),
        }
    }

    pub fn add_num_term(&mut self, term: VdBsqNumTerm<'sess>) {
        match term {
            VdBsqNumTerm::Rnum(term) => {
                self.constant_rnum += term;
            }
            VdBsqNumTerm::Inum(term) => match term {
                VdBsqInumTerm::Atom(term) => self.add_atom(term),
                VdBsqInumTerm::Sum(term) => self.add_sum(term),
                VdBsqInumTerm::Product(rnum, term) => self.add_product(rnum, term),
            },
        }
    }

    pub fn sub_num_term(&mut self, term: VdBsqNumTerm<'sess>) {
        match term {
            VdBsqNumTerm::Rnum(term) => self.constant_rnum -= term,
            VdBsqNumTerm::Inum(term) => todo!(),
        }
    }

    pub fn add_atom(&mut self, term: VdBsqAtomInumTerm<'sess>) {
        self.add_monomial(VdBsqNonSumInumTerm::Atom(term), VdBsqRnumTerm::ONE);
    }

    pub fn add_sum(&mut self, term: VdBsqSumInumTerm<'sess>) {
        todo!()
    }

    pub fn add_product(&mut self, rnum: VdBsqRnumTerm, term: VdBsqProductInumTermBase<'sess>) {
        todo!()
    }

    pub fn add_monomial(&mut self, term: VdBsqNonSumInumTerm<'sess>, coeff: VdBsqRnumTerm) {
        self.unpruned_monomials
            .insert_or_update((term, coeff), |(_, old_coeff)| {
                *old_coeff += coeff;
            });
    }

    pub fn finish(self) -> VdBsqNumTerm<'sess> {
        use husky_print_utils::*;
        p!(self.constant_rnum, self.unpruned_monomials);
        let monomials: VdBsqInumMonomialCoefficients = self
            .unpruned_monomials
            .into_iter()
            .filter(|(_, coeff)| !coeff.is_zero())
            .collect();
        match (monomials.len(), self.constant_rnum) {
            (0, _) => self.constant_rnum.into(),
            (1, VdBsqRnumTerm::ZERO) => {
                todo!()
                // let (term, coeff) = monomials.into_iter().next().unwrap();
                // VdBsqNumTerm::Inum(VdBsqInumTerm::Product(self.constant_rnum, term))
            }
            _ => VdBsqSumInumTerm::new(self.constant_rnum, monomials, self.db).into(),
        }
    }
}
