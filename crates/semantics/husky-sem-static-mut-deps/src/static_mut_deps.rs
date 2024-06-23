use husky_entity_path::path::ItemPath;
use vec_like::OrderedSmallVecSet;

#[salsa::derive_debug_with_db]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SemStaticMutDeps(OrderedSmallVecSet<ItemPath, 4>);

impl SemStaticMutDeps {
    pub(crate) fn merge(&mut self, other: &Self) {
        self.0.extend(&other.0);
    }
}
