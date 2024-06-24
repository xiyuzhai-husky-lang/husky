use husky_entity_path::path::ItemPath;
use vec_like::OrderedSmallVecSet;

#[salsa::derive_debug_with_db]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SemStaticMutDeps(OrderedSmallVecSet<ItemPath, 4>);

impl SemStaticMutDeps {
    /// returns whether `self` is changed
    pub(crate) fn merge(&mut self, other: &Self, counter: &mut EffectiveMergeCounter) {
        let old_len = self.0.len();
        self.0.extend(&other.0);
        if old_len != self.0.len() {
            counter.count += 1
        }
    }

    pub(crate) fn insert(&mut self, item_path: ItemPath) {
        self.0.insert(item_path);
    }
}

#[derive(Default)]
pub(crate) struct EffectiveMergeCounter {
    count: usize,
}

impl EffectiveMergeCounter {
    pub fn count(&self) -> usize {
        self.count
    }
}
