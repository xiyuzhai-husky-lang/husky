use common::*;
use stdx::sync::ARwLock;

use crate::{any::Any, *};

use super::{
    eval::Evaluator,
    feature::{Feature, FeatureId, FeatureKind},
    value::{CachedValue, DurableValue},
    *,
};

impl<'sess> Session<'sess> {
    pub(super) fn online_eval<'a>(
        &'a self,
        feature_id: FeatureId,
        input: &'a dyn Any,
    ) -> Option<Box<dyn Any>> {
        OnlineEvaluator::new(self, input)
            .eval(feature_id)
            .clone_any()
    }
}

struct OnlineEvaluator<'eval, 'sess: 'eval> {
    sess: &'eval Session<'sess>,
    input: &'eval dyn Any,
    cached_values: ARwLock<HashMap<FeatureId, CachedValue<'eval>>>,
}

impl<'eval, 'sess: 'eval> OnlineEvaluator<'eval, 'sess> {
    pub(super) fn new(sess: &'eval Session<'sess>, input: &'eval dyn Any) -> Self {
        Self {
            sess,
            input,
            cached_values: ARwLock::new(HashMap::new()),
        }
    }
}

impl<'eval, 'sess: 'eval> Evaluator<'eval> for OnlineEvaluator<'eval, 'sess> {
    fn feature_kind(&self, feature_id: FeatureId) -> &'eval FeatureKind {
        &self.sess.features[feature_id.0].kind
    }

    fn cache(
        &self,
        feature_id: FeatureId,
        cached_value: CachedValue<'eval>,
    ) -> DurableValue<'eval> {
        let value = unsafe { cached_value.value() };
        self.cached_values
            .write(|values| should!(values.insert(feature_id, cached_value).is_none()));
        value
    }
}
