use common::*;

use crate::{any::Any, virtual_stack::VirtualStackValue, *};

use super::*;

impl<'sess> Session<'sess> {
    fn intern(&mut self, kind: FeatureKind) -> FeatureId {
        if let Some(id) = self.feature_ids.get(&kind) {
            *id
        } else {
            let id = FeatureId(self.features.len());
            self.features.push(self.train(Feature::new(kind.clone())));
            self.feature_ids.insert(kind, id);
            id
        }
    }
}
