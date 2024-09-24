pub mod control_transfer;
pub mod value;

use husky_entity_path::path::ItemPath;
use vec_like::OrderedSmallVecSet;

#[derive(Default)]
pub(crate) struct EffectiveMergeCounter {
    count: usize,
}

impl EffectiveMergeCounter {
    pub fn count(&self) -> usize {
        self.count
    }
}
