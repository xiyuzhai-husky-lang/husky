use common::*;

use crate::*;
use vm::StackValue;

use super::*;

impl<'sess> Session<'sess> {
    pub(super) fn intern_feature(&mut self, feature: Feature, cached: bool) -> FeatureId {
        if let Some(id) = self.features.ids.get(&feature) {
            *id
        } else {
            let id = FeatureId {
                raw: self.features.features.len(),
                cached,
            };
            self.features.features.push(self.train(feature.clone()));
            self.features.ids.insert(feature, id);
            id
        }
    }
}
