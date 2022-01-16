use common::*;

use crate::{any::Any, virtual_stack::VirtualStackValue, *};

use super::{
    eval::Evaluator,
    feature::{Feature, FeatureId, FeatureKind},
    value::{CachedValue, DurableValue},
    *,
};

impl<'sess> Session<'sess> {
    pub(super) fn offline_eval(
        &self,
        feature_id: FeatureId,
        input_idx: usize,
    ) -> VirtualStackValue {
        OfflineEvaluator::new(self, input_idx).eval(feature_id)
    }

    fn cache(
        &self,
        feature_id: FeatureId,
        input_idx: usize,
        value: CachedValue<'sess>,
    ) -> DurableValue<'sess> {
        self.features[feature_id.0].cache(input_idx, value)
    }
}

struct OfflineEvaluator<'a, 'sess: 'a> {
    sess: &'a Session<'sess>,
    input_idx: usize,
}

impl<'a, 'sess: 'a> OfflineEvaluator<'a, 'sess> {
    fn new(sess: &'a Session<'sess>, input_idx: usize) -> Self {
        Self { sess, input_idx }
    }
}

impl<'a, 'sess: 'a> Evaluator<'sess> for OfflineEvaluator<'a, 'sess> {
    fn feature_kind(&self, feature_id: FeatureId) -> &FeatureKind {
        &self.sess.features[feature_id.0].kind
    }

    fn cache(&self, feature_id: FeatureId, value: CachedValue<'sess>) -> DurableValue<'sess> {
        self.sess.cache(feature_id, self.input_idx, value)
    }
}
