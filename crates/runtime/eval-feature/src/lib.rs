mod session;

pub use session::*;

use compile_time_db::FeatureQueryGroup;
use feature::{
    eval_feature_expr, eval_feature_lazy_block, eval_feature_repr, eval_feature_stmt, FeatureExpr,
    FeatureLazyBlock, FeatureRepr, FeatureStmt,
};
use std::{
    borrow::Cow,
    collections::HashMap,
    sync::{Arc, Mutex},
};
use vm::EvalResult;

pub trait EvalFeature<'eval> {
    fn feature_query_group(&self) -> &dyn FeatureQueryGroup;
    fn session(&self) -> &Arc<Mutex<Session<'eval>>>;
    fn verbose(&self) -> bool;

    fn eval_feature_repr(&self, repr: &FeatureRepr, input_id: usize) -> EvalResult<'eval> {
        let dev = &mut self.session().lock().unwrap().dev;
        let sheet = &mut dev.sheets[input_id];
        let input = dev.loader.load(input_id).input;
        eval_feature_repr(
            self.feature_query_group(),
            repr,
            input,
            sheet,
            self.verbose(),
        )
    }

    fn eval_feature_lazy_block(
        &self,
        block: &FeatureLazyBlock,
        input_id: usize,
    ) -> EvalResult<'eval> {
        let dev = &mut self.session().lock().unwrap().dev;
        let sheet = &mut dev.sheets[input_id];
        let input = dev.loader.load(input_id).input;
        eval_feature_lazy_block(
            self.feature_query_group(),
            block,
            input,
            sheet,
            self.verbose(),
        )
    }

    fn eval_feature_stmt(&self, stmt: &FeatureStmt, input_id: usize) -> EvalResult<'eval> {
        let dev = &mut self.session().lock().unwrap().dev;
        let sheet = &mut dev.sheets[input_id];
        let input = dev.loader.load(input_id).input;
        eval_feature_stmt(
            self.feature_query_group(),
            stmt,
            input,
            sheet,
            self.verbose(),
        )
    }

    fn eval_feature_expr(&self, expr: &FeatureExpr, input_id: usize) -> EvalResult<'eval> {
        let dev = &mut self.session().lock().unwrap().dev;
        let sheet = &mut dev.sheets[input_id];
        let input = dev.loader.load(input_id).input;
        eval_feature_expr(
            self.feature_query_group(),
            expr,
            input,
            sheet,
            self.verbose(),
        )
    }
}
