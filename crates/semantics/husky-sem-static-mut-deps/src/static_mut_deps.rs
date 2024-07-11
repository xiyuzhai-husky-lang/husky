use husky_entity_path::path::ItemPath;
use vec_like::OrderedSmallVecSet;

#[salsa::derive_debug_with_db]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SemStaticMutDeps(OrderedSmallVecSet<ItemPath, 4>);

impl SemStaticMutDeps {
    pub(crate) fn merge(&mut self, other: &Self) {
        self.0.extend(&other.0);
    }

    pub(crate) fn merge_counted(&mut self, other: &Self, counter: &mut EffectiveMergeCounter) {
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

impl std::ops::Deref for SemStaticMutDeps {
    type Target = OrderedSmallVecSet<ItemPath, 4>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoIterator for &SemStaticMutDeps {
    type Item = ItemPath;

    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().copied()
    }
}
