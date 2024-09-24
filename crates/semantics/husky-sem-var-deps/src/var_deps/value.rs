use super::SemVarDep;
use vec_like::OrderedSmallVecSet;

#[salsa::derive_debug_with_db]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SemValueVarDeps(OrderedSmallVecSet<SemVarDep, 4>);

impl std::ops::Deref for SemValueVarDeps {
    type Target = OrderedSmallVecSet<SemVarDep, 4>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoIterator for &SemValueVarDeps {
    type Item = SemVarDep;

    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().copied()
    }
}

impl SemValueVarDeps {
    pub(crate) fn merge(&mut self, other: &[SemVarDep]) {
        self.0.extend(other);
    }

    pub(crate) fn merge_counted(
        &mut self,
        other: &[SemVarDep],
        counter: &mut super::EffectiveMergeCounter,
    ) {
        let old_len = self.0.len();
        self.0.extend(other);
        if old_len != self.0.len() {
            counter.count += 1
        }
    }

    pub(crate) fn insert_item_path(&mut self, item_path: super::ItemPath) {
        self.0.insert(SemVarDep::Item(item_path));
    }

    pub(crate) fn remove_item_path(&mut self, item_path: impl Into<super::ItemPath>) {
        self.0.remove(SemVarDep::Item(item_path.into()))
    }
}
