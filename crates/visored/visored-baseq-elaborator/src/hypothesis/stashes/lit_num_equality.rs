use super::*;
use crate::{
    hypothesis::stash::{
        unique::{IsVdBsqHypothesisUniqueStashScheme, VdBsqHypothesisUniqueStash},
        IsVdBsqHypothesisStashScheme,
    },
    term::{litnum::VdBsqLitnumTerm, VdBsqTerm},
};

pub type LitnumEqualityStash<'sess> = VdBsqHypothesisUniqueStash<'sess, VdBsqLitNumEqualityScheme>;

pub struct VdBsqLitNumEqualityScheme;

impl IsVdBsqHypothesisStashScheme for VdBsqLitNumEqualityScheme {
    type Key<'sess> = VdBsqLitNumEqualityKey<'sess>;

    type Value<'sess> = VdBsqLitNumEqualityValue<'sess>;
}

pub struct VdBsqLitNumEqualityKey<'sess> {
    reduced_term: VdBsqTerm<'sess>,
}

pub struct VdBsqLitNumEqualityValue<'sess> {
    litn: VdBsqLitnumTerm<'sess>,
}

impl IsVdBsqHypothesisUniqueStashScheme for VdBsqLitNumEqualityScheme {}
