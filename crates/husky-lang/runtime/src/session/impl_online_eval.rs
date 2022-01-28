use common::*;
use stdx::sync::ARwLock;
use vm::{BoxedValue, EvalValue};

use crate::*;

use super::{
    eval::Evaluator,
    feature::{Feature, FeatureId},
    *,
};

impl<'sess> Session<'sess> {
    pub(super) fn online_eval<'a>(
        &'a self,
        feature_id: FeatureId,
        input: &'a dyn AnyValueDyn,
    ) -> VMResult<BoxedValue> {
        todo!()
        // OnlineEvaluator::new(self, input).eval(feature_id).owned()
    }
}

struct OnlineEvaluator<'eval, 'sess: 'eval> {
    sess: &'eval Session<'sess>,
    input: &'eval dyn AnyValueDyn,
    cache: EvalCache<'eval>,
}

impl<'eval, 'sess: 'eval> OnlineEvaluator<'eval, 'sess> {
    pub(super) fn new(sess: &'eval Session<'sess>, input: &'eval dyn AnyValueDyn) -> Self {
        Self {
            sess,
            input,
            cache: Default::default(),
        }
    }
}
