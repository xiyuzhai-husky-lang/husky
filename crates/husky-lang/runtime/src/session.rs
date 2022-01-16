mod eval;
mod feature;
mod impl_offline_eval;
mod impl_online_eval;
mod impl_train;
mod tests;
mod value;

use common::*;

use crate::{any::Any, *};

use dataset::Dataset;

use feature::{Feature, FeatureId, FeatureKind};
use value::CachedValue;

pub struct Session<'sess> {
    dataset: &'sess dyn Dataset,
    features: Vec<Feature<'sess>>,
    feature_ids: HashMap<FeatureKind, FeatureId>,
}

impl<'sess> Session<'sess> {
    pub(crate) fn intern(&mut self, kind: FeatureKind) -> FeatureId {
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
