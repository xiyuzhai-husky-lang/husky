mod evaluator;
mod query;
mod session;

pub use evaluator::*;
use husky_tracer_protocol::VisualData;
pub use query::*;
pub use session::*;

use feature_gen::*;
use husky_compile_time::FeatureGenQueryGroup;
use std::{
    borrow::Cow,
    collections::HashMap,
    sync::{Arc, Mutex},
};
use upcast::Upcast;
use vm::{EvalResult, VMRuntimeResult};

pub trait EvalFeature<'eval>: FeatureEvalQueryGroup + Upcast<dyn FeatureEvalQueryGroup> {
    fn session(&self) -> &Session<'eval>;
    fn verbose(&self) -> bool;

    fn evaluator<'a>(&'a self, sample_id: usize) -> FeatureEvaluator<'a, 'eval> {
        let dev = self.session().dev();
        let sheet = &dev.sheets[sample_id];
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

    fn visualize(&self, this: FeatureRepr, input_id: usize) -> VMRuntimeResult<VisualData>
    where
        'eval: 'static,
    {
        self.evaluator(input_id).visualize(this)
    }

    fn eval_feature_repr(&self, repr: &FeatureRepr, input_id: usize) -> EvalResult<'eval> {
        self.evaluator(input_id).eval_feature_repr(repr)
    }

    fn eval_feature_repr_cached(&self, repr: &FeatureRepr, input_id: usize) -> EvalResult<'eval> {
        self.evaluator(input_id).eval_feature_repr_cached(repr)
    }

    fn eval_feature_lazy_block(
        &self,
        block: &FeatureLazyBlock,
        input_id: usize,
    ) -> EvalResult<'eval> {
        self.evaluator(input_id).eval_feature_lazy_block(block)
    }

    fn eval_feature_stmt(&self, stmt: &FeatureStmt, input_id: usize) -> EvalResult<'eval> {
        self.evaluator(input_id).eval_feature_stmt(stmt)
    }

    fn eval_feature_expr(&self, expr: &FeatureLazyExpr, input_id: usize) -> EvalResult<'eval> {
        self.evaluator(input_id).eval_feature_lazy_expr(expr)
    }
}
