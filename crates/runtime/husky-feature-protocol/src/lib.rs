use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FeatureId(usize);

impl FeatureId {
    pub fn new(raw: usize) -> Self {
        FeatureId(raw)
    }

    pub fn raw(self) -> usize {
        self.0
    }
}
