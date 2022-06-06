mod evaluator;
mod query;
mod session;

pub use evaluator::*;
pub use query::*;
pub use session::*;

use compile_time_db::FeatureGenQueryGroup;
use feature::*;
use std::{
    borrow::Cow,
    collections::HashMap,
    sync::{Arc, Mutex},
};
use upcast::Upcast;
use vm::EvalResult;

pub trait EvalFeature<'eval>: FeatureEvalQueryGroup + Upcast<dyn FeatureEvalQueryGroup> {
    fn session(&self) -> &Arc<Mutex<Session<'eval>>>;
    fn verbose(&self) -> bool;

    fn eval_feature_repr(&self, repr: &FeatureRepr, input_id: usize) -> EvalResult<'eval> {
        let dev = &mut self.session().lock().unwrap().dev;
        let sheet = &mut dev.sheets[input_id];
        let input = dev.loader.load(input_id).input;
        eval_feature_repr(self.upcast(), repr, input, sheet, self.verbose())
    }

    fn eval_feature_lazy_block(
        &self,
        block: &FeatureLazyBlock,
        input_id: usize,
    ) -> EvalResult<'eval> {
        let dev = &mut self.session().lock().unwrap().dev;
        let sheet = &mut dev.sheets[input_id];
        let input = dev.loader.load(input_id).input;
        eval_feature_lazy_block(self.upcast(), block, input, sheet, self.verbose())
    }

    fn eval_feature_stmt(&self, stmt: &FeatureStmt, input_id: usize) -> EvalResult<'eval> {
        let dev = &mut self.session().lock().unwrap().dev;
        let sheet = &mut dev.sheets[input_id];
        let input = dev.loader.load(input_id).input;
        eval_feature_stmt(self.upcast(), stmt, input, sheet, self.verbose())
    }

    fn eval_feature_expr(&self, expr: &FeatureExpr, input_id: usize) -> EvalResult<'eval> {
        let dev = &mut self.session().lock().unwrap().dev;
        let sheet = &mut dev.sheets[input_id];
        let input = dev.loader.load(input_id).input;
        eval_feature_expr(self.upcast(), expr, input, sheet, self.verbose())
    }
}
