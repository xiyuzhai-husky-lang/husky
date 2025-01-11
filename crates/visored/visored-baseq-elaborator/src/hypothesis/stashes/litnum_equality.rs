use super::*;
use crate::{
    hypothesis::{
        stack::VdBsqHypothesisStack,
        stash::{
            unique::{IsVdBsqHypothesisUniqueStashScheme, VdBsqHypothesisUniqueStash},
            IsVdBsqHypothesisStashScheme,
        },
    },
    term::{
        builder::{product::VdBsqProductBuilder, sum::VdBsqSumBuilder},
        comnum::{sum::VdBsqSumComnumTerm, VdBsqComnumTerm, VdBsqMonomialCoefficients},
        litnum::VdBsqLitnumTerm,
        num::VdBsqNumTerm,
        prop::{num_relationship::VdBsqNumRelationshipPropTermKind, VdBsqPropTerm},
        VdBsqTerm,
    },
};
use floated_sequential::db::FloaterDb;
use husky_control_flow_utils::require;

pub type LitnumEqualityStash<'sess> = VdBsqHypothesisUniqueStash<'sess, VdBsqLitNumEqualityScheme>;

pub struct VdBsqLitNumEqualityScheme;

impl IsVdBsqHypothesisStashScheme for VdBsqLitNumEqualityScheme {
    type Key<'sess> = VdBsqLitNumEqualityKey<'sess>;

    type Value<'sess> = VdBsqLitNumEqualityValue<'sess>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VdBsqLitNumEqualityKey<'sess> {
    normalized_monomials: VdBsqNumTerm<'sess>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VdBsqLitNumEqualityValue<'sess> {
    litnum: VdBsqLitnumTerm<'sess>,
}

impl<'sess> VdBsqLitNumEqualityValue<'sess> {
    pub fn litnum(&self) -> VdBsqLitnumTerm<'sess> {
        self.litnum
    }
}

impl IsVdBsqHypothesisUniqueStashScheme for VdBsqLitNumEqualityScheme {
    fn key_value_from_hypothesis<'sess>(
        record: VdBsqHypothesisStackRecord<'sess>,
        entry: &VdBsqHypothesisEntry<'sess>,
        db: &'sess FloaterDb,
    ) -> Option<(Self::Key<'sess>, Self::Value<'sess>)> {
        let VdBsqTerm::Prop(VdBsqPropTerm::NumRelationship(term)) = entry.expr().term() else {
            return None;
        };
        require!(term.kind() == VdBsqNumRelationshipPropTermKind::Eq);
        require!(let VdBsqNumTerm::Comnum(VdBsqComnumTerm::Sum(lhs_minus_rhs)) = term.lhs_minus_rhs());
        let (_, (normalized_constant_litnum, normalized_monomials)) =
            split_sum_into_normalized_litnum_and_monomials_fld(lhs_minus_rhs, db);
        let neg_normalized_constant_litnum = normalized_constant_litnum.neg(db);
        let key = VdBsqLitNumEqualityKey {
            normalized_monomials,
        };
        let value = VdBsqLitNumEqualityValue {
            litnum: neg_normalized_constant_litnum,
        };
        Some((key, value))
    }
}

fn split_sum_into_normalized_litnum_and_monomials_fld<'sess>(
    sum: VdBsqSumComnumTerm<'sess>,
    db: &'sess FloaterDb,
) -> (
    VdBsqLitnumTerm<'sess>,
    (VdBsqLitnumTerm<'sess>, VdBsqNumTerm<'sess>),
) {
    let (factor, (litnum, monomials)) = split_sum_into_normalized_litnum_and_monomials(sum, db);
    let monomials = if monomials.len() > 1 {
        VdBsqSumComnumTerm::new(0, monomials, db).into()
    } else {
        let (monomial, coeff) = monomials.data()[0];
        coeff.mul_nonsum(monomial, db)
    };
    (factor, (litnum, monomials))
}

fn split_sum_into_normalized_litnum_and_monomials<'sess>(
    sum: VdBsqSumComnumTerm<'sess>,
    db: &'sess FloaterDb,
) -> (
    VdBsqLitnumTerm<'sess>,
    (VdBsqLitnumTerm<'sess>, VdBsqMonomialCoefficients<'sess>),
) {
    let mut monomials = sum.monomials().clone();
    debug_assert!(monomials.len() > 0);
    let coeff0 = monomials.data()[0].1;
    debug_assert!(coeff0.is_nonzero());
    let inv_coeff0 = coeff0.inverse().expect("nonzero");
    let normalized_constant_term = sum.constant_term().mul(inv_coeff0, db);
    let normalized_monomials = monomials.map_collect(|coeff| coeff.mul(inv_coeff0, db));
    (inv_coeff0, (normalized_constant_term, normalized_monomials))
}

impl<'sess> LitnumEqualityStash<'sess> {
    pub fn reduce(
        &self,
        term: VdBsqComnumTerm<'sess>,
        active_hypotheses: &VdBsqActiveHypotheses<'sess>,
        db: &'sess FloaterDb,
    ) -> Option<VdBsqLitnumTerm<'sess>> {
        /// decompose `t = a(b + x)`
        let (a, (b, x)): (
            VdBsqLitnumTerm<'sess>,
            (VdBsqLitnumTerm<'sess>, VdBsqNumTerm<'sess>),
        ) = match term {
            VdBsqComnumTerm::Atom(atom) => {
                (VdBsqLitnumTerm::ONE, (VdBsqLitnumTerm::ZERO, atom.into()))
            }
            VdBsqComnumTerm::Sum(sum) => {
                split_sum_into_normalized_litnum_and_monomials_fld(sum, db)
            }
            VdBsqComnumTerm::Product(term, base) => {
                todo!()
            }
        };
        let key = VdBsqLitNumEqualityKey {
            normalized_monomials: x,
        };
        let value = self.get_valid_value(&key, active_hypotheses)?.litnum;
        Some(a.mul(value.add(b, db), db))
    }
}
