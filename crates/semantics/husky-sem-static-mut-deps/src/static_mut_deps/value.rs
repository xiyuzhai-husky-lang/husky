use super::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SemValueStaticMutDeps(pub(super) OrderedSmallVecSet<ItemPath, 4>);

impl SemValueStaticMutDeps {
    pub(crate) fn merge(&mut self, other: &[ItemPath]) {
        self.0.extend(other);
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

impl std::ops::Deref for SemValueStaticMutDeps {
    type Target = OrderedSmallVecSet<ItemPath, 4>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoIterator for &SemValueStaticMutDeps {
    type Item = ItemPath;

    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().copied()
    }
}
