mod evaluator;
mod session;

pub use evaluator::*;
use husky_entity_route::EntityRoutePtr;
use husky_trace_protocol::{SampleId, VisualData};
pub use session::*;

use husky_feature_gen::*;
use std::{
    borrow::Cow,
    collections::HashMap,
    sync::{Arc, Mutex},
};
use upcast::Upcast;
use vm::{VMConfig, __EvalResult, __EvalValue, __EvalValueResult};

pub trait EvalFeature<'eval>: FeatureGenQueryGroup + Upcast<dyn FeatureGenQueryGroup> {
    fn session(&self) -> &Session<'eval>;
    fn vm_config(&self) -> &VMConfig;

    fn evaluator<'a>(&'a self, sample_id: SampleId) -> FeatureEvaluator<'a, 'eval> {
        let dev = self.session().dev();
        let sheet = &dev.sheets[sample_id.0];
        let eval_input = dev.load(sample_id).input;
        FeatureEvaluator {
            sample_id,
            db: self.upcast(),
            eval_input,
            sheet,
            vm_config: self.vm_config(),
            opt_static_husky_feature_eval: self.opt_static_husky_feature_eval(),
        }
    }

    // None for 'eval is shorter than 'static
    // Some(self) otherwise
    fn opt_static_husky_feature_eval(&self) -> Option<&dyn EvalFeature<'static>>;

    fn visualize_feature(&self, this: FeatureRepr, sample_id: SampleId) -> __EvalResult<VisualData>
    where
        'eval: 'static,
    {
        self.evaluator(sample_id).visualize_feature(this)
    }

    fn husky_feature_eval_repr(
        &self,
        repr: &FeatureRepr,
        sample_id: SampleId,
    ) -> __EvalValueResult<'eval> {
        self.evaluator(sample_id).husky_feature_eval_repr(repr)
    }

    fn husky_feature_eval_repr_cached(
        &self,
        repr: &FeatureRepr,
        sample_id: SampleId,
    ) -> __EvalValueResult<'eval> {
        self.evaluator(sample_id)
            .husky_feature_eval_repr_cached(repr)
    }

    fn husky_feature_eval_lazy_block(
        &self,
        block: &FeatureLazyBlock,
        sample_id: SampleId,
    ) -> __EvalValueResult<'eval> {
        self.evaluator(sample_id)
            .husky_feature_eval_lazy_block(block)
    }

    fn husky_feature_eval_stmt(
        &self,
        stmt: &FeatureStmt,
        sample_id: SampleId,
    ) -> __EvalResult<__EvalValue<'eval>> {
        self.evaluator(sample_id).husky_feature_eval_stmt(stmt)
    }

    fn husky_feature_eval_expr(
        &self,
        expr: &FeatureExpr,
        sample_id: SampleId,
    ) -> __EvalValueResult<'eval> {
        self.evaluator(sample_id).husky_feature_eval_expr(expr)
    }
}
