pub mod control_flow;
pub mod control_transfer;
pub mod domain;
pub mod value;

use husky_entity_path::path::ItemPath;
use vec_like::OrderedSmallVecSet;

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub enum SemVarDep {
    Item(ItemPath),
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
