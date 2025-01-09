use super::*;
use crate::hypothesis::{stack::VdBsqHypothesisStackRecord, VdBsqHypothesisEntry};
use floated_sequential::db::FloaterDb;
use rustc_hash::FxHashMap;
use std::marker::PhantomData;

pub struct VdBsqHypothesisUniqueStash<'sess, Scheme>
where
    Scheme: IsVdBsqHypothesisUniqueStashScheme,
{
    map: FxHashMap<Scheme::Key<'sess>, (VdBsqHypothesisStackRecord<'sess>, Scheme::Value<'sess>)>,
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
    pub(crate) fn new() -> Self {
        Self {
            map: FxHashMap::default(),
        }
    }
}

impl<'sess, Scheme> VdBsqHypothesisUniqueStash<'sess, Scheme>
where
    Scheme: IsVdBsqHypothesisUniqueStashScheme,
{
    pub(crate) fn add_hypothesis(
        &mut self,
        hypothesis_record: VdBsqHypothesisStackRecord<'sess>,
        hypothesis_entry: &VdBsqHypothesisEntry<'sess>,
        db: &'sess FloaterDb,
    ) {
        let Some((key, value)) =
            Scheme::key_value_from_hypothesis(hypothesis_record, hypothesis_entry, db)
        else {
            return;
        };
        if let Some((existing_record, existing_value)) = self.map.get(&key) {
            assert_eq!(existing_value, &value);
        } else {
            debug_assert!(self.map.insert(key, (hypothesis_record, value)).is_none());
        }
    }
}
