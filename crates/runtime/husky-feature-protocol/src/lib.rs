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

impl std::fmt::Display for FeatureId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ft#{}", self.0)
    }
}
