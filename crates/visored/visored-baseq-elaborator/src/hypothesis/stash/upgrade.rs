use super::*;
use crate::hypothesis::{
    stack::{VdBsqActiveHypotheses, VdBsqHypothesisStackRecord},
    VdBsqHypothesisEntry, VdBsqHypothesisIdx,
};
use floated_sequential::db::FloaterDb;
use rustc_hash::FxHashMap;
use smallvec::*;
use std::marker::PhantomData;

pub trait IsVdBsqHypothesisUpgradeStashScheme: IsVdBsqHypothesisStashScheme {
    fn is_new_value_an_upgrade<'sess>(old: &Self::Value<'sess>, new: &Self::Value<'sess>) -> bool;
    fn key_value_from_hypothesis<'sess>(
        record: VdBsqHypothesisStackRecord<'sess>,
        entry: &VdBsqHypothesisEntry<'sess>,
        db: &'sess FloaterDb,
    ) -> Option<(Self::Key<'sess>, Self::Value<'sess>)>;
}

pub struct VdBsqHypothesisUpgradeStash<'sess, Scheme>
where
    Scheme: IsVdBsqHypothesisUpgradeStashScheme,
{
    map: FxHashMap<Scheme::Key<'sess>, VdBsqHypothesisUpgradeStashEntry<'sess, Scheme>>,
}

pub struct VdBsqHypothesisUpgradeStashEntry<'sess, Scheme>
where
    Scheme: IsVdBsqHypothesisUpgradeStashScheme,
{
    values: SmallVec<[(VdBsqHypothesisStackRecord<'sess>, Scheme::Value<'sess>); 4]>,
}

impl<'sess, Scheme> Default for VdBsqHypothesisUpgradeStashEntry<'sess, Scheme>
where
    Scheme: IsVdBsqHypothesisUpgradeStashScheme,
{
    fn default() -> Self {
        Self {
            values: smallvec![],
        }
    }
}

impl<'sess, Scheme> Default for VdBsqHypothesisUpgradeStash<'sess, Scheme>
where
    Scheme: IsVdBsqHypothesisUpgradeStashScheme,
{
    fn default() -> Self {
        Self {
            map: FxHashMap::default(),
        }
    }
}

impl<'sess, Scheme> VdBsqHypothesisUpgradeStashEntry<'sess, Scheme>
where
    Scheme: IsVdBsqHypothesisUpgradeStashScheme,
{
    fn cache(
        &mut self,
        hypothesis_stack_record: VdBsqHypothesisStackRecord<'sess>,
        active_hypotheses: &VdBsqActiveHypotheses,
        value: Scheme::Value<'sess>,
    ) {
        self.clear_inactive_values(active_hypotheses);
        match self.values.last() {
            Some((_, last_value)) if Scheme::is_new_value_an_upgrade(last_value, &value) => {
                self.values.push((hypothesis_stack_record, value));
            }
            None => {
                self.values.push((hypothesis_stack_record, value));
            }
            _ => (),
        }
    }

    fn clear_inactive_values(&mut self, active_hypotheses: &VdBsqActiveHypotheses) {
        while let Some(&(stack_record, _)) = self.values.last()
            && !active_hypotheses.is_record_active(stack_record)
        {
            self.values.pop();
        }
    }
}

impl<'sess, Scheme> VdBsqHypothesisUpgradeStash<'sess, Scheme>
where
    Scheme: IsVdBsqHypothesisUpgradeStashScheme,
{
    pub fn new() -> Self {
        Self {
            map: FxHashMap::default(),
        }
    }
}

impl<'sess, Scheme> VdBsqHypothesisUpgradeStash<'sess, Scheme>
where
    Scheme: IsVdBsqHypothesisUpgradeStashScheme,
{
    pub fn add_hypothesis(
        &mut self,
        hypothesis_stack_record: VdBsqHypothesisStackRecord<'sess>,
        hypothesis_entry: &VdBsqHypothesisEntry<'sess>,
        db: &'sess FloaterDb,
        active_hypotheses: &VdBsqActiveHypotheses<'sess>,
    ) {
        let Some((key, value)) =
            Scheme::key_value_from_hypothesis(hypothesis_stack_record, hypothesis_entry, db)
        else {
            return;
        };
        self.map
            .entry(key)
            .or_default()
            .cache(hypothesis_stack_record, active_hypotheses, value);
    }
}
