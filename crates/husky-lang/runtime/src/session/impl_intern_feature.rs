use common::*;

use crate::*;
use vm::StackValue;

use super::*;

impl<'sess> Session<'sess> {
    pub(super) fn intern_feature(&mut self, feature: Feature) -> FeatureId {
        if let Some(id) = self.features.ids.get(&feature) {
            *id
        } else {
            let id = FeatureId(self.features.features.len());
            self.features.features.push(self.train(feature.clone()));
            self.features.ids.insert(feature, id);
            id
        }
    }
}
