mod evaluator;
mod session;

pub use evaluator::*;
use husky_tracer_protocol::{SampleId, VisualData};
pub use session::*;

use feature_gen::*;
use std::{
    borrow::Cow,
    collections::HashMap,
    sync::{Arc, Mutex},
};
use upcast::Upcast;
use vm::{EvalResult, VMRuntimeResult};

pub trait EvalFeature<'eval>: FeatureGenQueryGroup + Upcast<dyn FeatureGenQueryGroup> {
    fn session(&self) -> &Session<'eval>;
    fn verbose(&self) -> bool;

    fn evaluator<'a>(&'a self, sample_id: SampleId) -> FeatureEvaluator<'a, 'eval> {
        let dev = self.session().dev();
        let sheet = &dev.sheets[sample_id.0];
        let eval_input = dev.load(sample_id).input;
        FeatureEvaluator {
            sample_id,
            db: self.upcast(),
            eval_input,
            sheet,
            verbose: self.verbose(),
            opt_static_eval_feature: self.opt_static_eval_feature(),
        }
    }

    // None for 'eval is shorter than 'static
    // Some(self) otherwise
    fn opt_static_eval_feature(&self) -> Option<&dyn EvalFeature<'static>>;

    fn visualize(&self, this: FeatureRepr, sample_id: SampleId) -> VMRuntimeResult<VisualData>
    where
        'eval: 'static,
    {
        self.evaluator(sample_id).visualize(this)
    }

    fn eval_feature_repr(&self, repr: &FeatureRepr, sample_id: SampleId) -> EvalResult<'eval> {
        self.evaluator(sample_id).eval_feature_repr(repr)
    }

    fn eval_feature_repr_cached(
        &self,
        repr: &FeatureRepr,
        sample_id: SampleId,
    ) -> EvalResult<'eval> {
        self.evaluator(sample_id).eval_feature_repr_cached(repr)
    }

    fn eval_feature_lazy_block(
        &self,
        block: &FeatureLazyBlock,
        sample_id: SampleId,
    ) -> EvalResult<'eval> {
        self.evaluator(sample_id).eval_feature_lazy_block(block)
    }

    fn eval_feature_stmt(&self, stmt: &FeatureStmt, sample_id: SampleId) -> EvalResult<'eval> {
        self.evaluator(sample_id).eval_feature_stmt(stmt)
    }

    fn eval_feature_lazy_expr(
        &self,
        expr: &FeatureLazyExpr,
        sample_id: SampleId,
    ) -> EvalResult<'eval> {
        self.evaluator(sample_id).eval_feature_lazy_expr(expr)
    }
}
