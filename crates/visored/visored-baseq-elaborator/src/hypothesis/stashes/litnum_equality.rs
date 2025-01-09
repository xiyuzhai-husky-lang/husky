use super::*;
use crate::{
    hypothesis::stash::{
        unique::{IsVdBsqHypothesisUniqueStashScheme, VdBsqHypothesisUniqueStash},
        IsVdBsqHypothesisStashScheme,
    },
    term::{
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
    reduced_term: VdBsqTerm<'sess>,
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
        let (constant_term, monomials) = normalize_then_split(lhs_minus_rhs, db);
        todo!()
    }
}

fn normalize_then_split<'sess>(
    sum: VdBsqSumComnumTerm<'sess>,
    db: &'sess FloaterDb,
) -> (VdBsqLitnumTerm<'sess>, VdBsqSumComnumTerm<'sess>) {
    todo!()
}

fn normalize_then_split_aux<'sess>(
    sum: VdBsqSumComnumTerm<'sess>,
    db: &'sess FloaterDb,
) -> (VdBsqLitnumTerm<'sess>, VdBsqMonomialCoefficients<'sess>) {
    let mut monomials = sum.monomials().clone();
    debug_assert!(monomials.len() > 0);
    let coeff0 = monomials.data()[0].1;
    debug_assert!(coeff0.is_nonzero());
    let _ = coeff0.inverse().expect("nonzero");
    todo!()
}
