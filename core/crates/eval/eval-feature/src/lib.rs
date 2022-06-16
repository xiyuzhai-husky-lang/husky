mod evaluator;
mod query;
mod session;

pub use evaluator::*;
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
use vm::EvalResult;

pub trait EvalFeature<'eval>: FeatureEvalQueryGroup + Upcast<dyn FeatureEvalQueryGroup> {
    fn session(&self) -> &Session<'eval>;
    fn verbose(&self) -> bool;

    fn eval_feature_repr(&self, repr: &FeatureRepr, input_id: usize) -> EvalResult<'eval> {
        let dev = self.session().dev();
        let sheet = &dev.sheets[input_id];
        let input = dev.load(input_id).input;
        eval_feature_repr(self.upcast(), repr, input, sheet, self.verbose())
    }

    fn eval_feature_lazy_block(
        &self,
        block: &FeatureLazyBlock,
        input_id: usize,
    ) -> EvalResult<'eval> {
        let dev = self.session().dev();
        let sheet = &dev.sheets[input_id];
        let input = dev.load(input_id).input;
        eval_feature_lazy_block(self.upcast(), block, input, sheet, self.verbose())
    }

    fn eval_feature_stmt(&self, stmt: &FeatureStmt, input_id: usize) -> EvalResult<'eval> {
        let dev = self.session().dev();
        let sheet = &dev.sheets[input_id];
        let input = dev.load(input_id).input;
        eval_feature_stmt(self.upcast(), stmt, input, sheet, self.verbose())
    }

    fn eval_feature_expr(&self, expr: &FeatureExpr, input_id: usize) -> EvalResult<'eval> {
        let dev = self.session().dev();
        let sheet = &dev.sheets[input_id];
        let input = dev.load(input_id).input;
        eval_feature_expr(self.upcast(), expr, input, sheet, self.verbose())
    }
}
