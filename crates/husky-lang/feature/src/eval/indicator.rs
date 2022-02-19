use bitvec::prelude::BitVec;

use super::id::FeatureEvalId;

#[derive(Debug, Default)]
pub struct FeatureEvalIndicator {
    v: BitVec,
}

impl FeatureEvalIndicator {
    pub fn set(&mut self, id: FeatureEvalId) {
        if id.0 >= self.v.len() {
            self.v.resize(id.0 + 1, false);
        }
        self.v.set(id.0, true)
    }
}
