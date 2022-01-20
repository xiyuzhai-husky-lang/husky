use common::*;

use crate::*;
use interpret::{Any, StackValue};

use super::{
    eval::Evaluator,
    feature::{Feature, FeatureId, FeatureKind},
    value::{CachedValue, CachedValueStorage},
    *,
};

impl<'sess> Session<'sess> {
    pub(super) fn offline_eval(&self, feature_id: FeatureId, input_idx: usize) -> StackValue {
        OfflineEvaluator::new(self, input_idx).eval(feature_id)
    }

    fn cache(
        &self,
        feature_id: FeatureId,
        input_idx: usize,
        value: CachedValueStorage<'sess>,
    ) -> CachedValue<'sess> {
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

    fn cache(&self, feature_id: FeatureId, value: CachedValueStorage<'sess>) -> CachedValue<'sess> {
        self.sess.cache(feature_id, self.input_idx, value)
    }
}
