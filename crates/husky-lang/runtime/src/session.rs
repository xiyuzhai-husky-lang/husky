mod feature;
mod impl_eval;
mod tests;
mod value;

use common::*;

use crate::{any::Any, *};

use dataset::Dataset;

use feature::{Feature, FeatureId, FeatureKind};
use value::SessionValue;

pub struct Session {
    dataset: Box<dyn Dataset>,
    features: Vec<Feature>,
    feature_ids: HashMap<FeatureKind, FeatureId>,
}

impl Session {
    pub(crate) fn intern(&mut self, kind: FeatureKind) -> FeatureId {
        if let Some(id) = self.feature_ids.get(&kind) {
            *id
        } else {
            let id = FeatureId(self.features.len());
            self.features.push(Feature::new(kind.clone()));
            self.feature_ids.insert(kind, id);
            id
        }
    }
}
