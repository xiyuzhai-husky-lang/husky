use std::ops::Index;

use crate::*;

pub trait InternFeature {
    fn features(&self) -> &FeatureInterner;
}

#[derive(Default)]
pub struct FeatureInterner {
    features: Vec<Feature>,
    ids: HashMap<Feature, FeatureId>,
}

impl Index<FeatureId> for FeatureInterner {
    type Output = Feature;

    fn index(&self, index: FeatureId) -> &Self::Output {
        &self.features[index.raw]
    }
}
