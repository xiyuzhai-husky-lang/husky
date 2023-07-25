use crate::*;
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct FeatureEvalIndicator {
    set: HashSet<FeatureEvalId>,
}

impl FeatureEvalIndicator {
    pub fn set(&mut self, id: FeatureEvalId) {
        self.set.insert(id);
    }
}
