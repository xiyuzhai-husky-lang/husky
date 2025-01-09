use super::*;
use crate::hypothesis::{stack::VdBsqHypothesisStackRecord, VdBsqHypothesisEntry};
use floated_sequential::db::FloaterDb;
use std::marker::PhantomData;

pub struct VdBsqHypothesisUniqueStash<'sess, Scheme>
where
    Scheme: IsVdBsqHypothesisUniqueStashScheme,
{
    phantom: PhantomData<&'sess (Scheme,)>,
}

pub trait IsVdBsqHypothesisUniqueStashScheme: IsVdBsqHypothesisStashScheme {
    fn key_value_from_hypothesis<'sess>(
        record: VdBsqHypothesisStackRecord<'sess>,
        entry: &VdBsqHypothesisEntry<'sess>,
        db: &'sess FloaterDb,
    ) -> Option<(Self::Key<'sess>, Self::Value<'sess>)>;
}

impl<'sess, Scheme> VdBsqHypothesisUniqueStash<'sess, Scheme>
where
    Scheme: IsVdBsqHypothesisUniqueStashScheme,
{
    pub(super) fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}
