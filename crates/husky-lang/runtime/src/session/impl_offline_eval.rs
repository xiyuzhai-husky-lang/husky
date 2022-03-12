use common::*;

use crate::*;
use vm::{AnyValueDyn, EvalValue, VMValue};

use super::{
    eval::Evaluator,
    feature::{Feature, FeatureId},
    *,
};

impl<'sess> Session<'sess> {
    pub(super) fn offline_eval(
        &self,
        feature_id: FeatureId,
        input_idx: usize,
    ) -> EvalValue<'sess, 'sess> {
        Evaluator::new(&self.features, &self.dev.caches[input_idx]).eval(feature_id)
    }
}
