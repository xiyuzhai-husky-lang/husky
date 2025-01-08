use super::*;
use crate::hypothesis::{
    stack::VdBsqHypothesisStackRecord, VdBsqHypothesisEntry, VdBsqHypothesisIdx,
};
use rustc_hash::FxHashMap;
use std::marker::PhantomData;

pub trait IsVdBsqHypothesisUpdateCacheScheme: IsVdBsqHypothesisCacheScheme {}

pub struct VdBsqHypothesisUpdateCache<'sess, Scheme>
where
    Scheme: IsVdBsqHypothesisUpdateCacheScheme,
{
    map: FxHashMap<Scheme::Key<'sess>, VdBsqHypothesisUpdateCacheEntry<'sess, Scheme>>,
}

pub struct VdBsqHypothesisUpdateCacheEntry<'sess, Scheme>
where
    Scheme: IsVdBsqHypothesisUpdateCacheScheme,
{
    values: Vec<(VdBsqHypothesisStackRecord<'sess>, Scheme::Value<'sess>)>,
}

impl<'sess, Scheme> Default for VdBsqHypothesisUpdateCache<'sess, Scheme>
where
    Scheme: IsVdBsqHypothesisUpdateCacheScheme,
{
    fn default() -> Self {
        Self {
            map: FxHashMap::default(),
        }
    }
}

impl<'sess, Scheme> VdBsqHypothesisUpdateCacheEntry<'sess, Scheme>
where
    Scheme: IsVdBsqHypothesisUpdateCacheScheme,
{
    fn clear_inactive_values(
        &mut self,
        stack_idx: usize,
        is_active: impl Fn(VdBsqHypothesisStackRecord<'sess>) -> bool,
    ) {
        while let Some(&(stack_record, _)) = self.values.last()
            && !is_active(stack_record)
        {
            self.values.pop();
        }
    }
}

impl<'sess, Scheme> VdBsqHypothesisUpdateCache<'sess, Scheme>
where
    Scheme: IsVdBsqHypothesisUpdateCacheScheme,
{
    pub fn cache(
        &mut self,
        hypothesis_record: VdBsqHypothesisStackRecord<'sess>,
        hypothesis_data: &VdBsqHypothesisEntry<'sess>,
    ) {
        todo!()
    }
}
