#![feature(trait_upcasting)]
pub mod db;
mod evaluator;
mod session;

pub use self::evaluator::*;
pub use self::session::*;
use husky_ethereal_term::EtherealTerm;
use husky_trace_protocol::{SampleId, VisualData};

use husky_val_repr::{db::ValReprDb, *};
use husky_vm::{RegularValue, VMConfig, VMResult};
use std::sync::{Arc, Mutex};

pub trait Runtime {
    fn db(&self) -> &dyn ValReprDb;
    fn session(&self) -> &Session;
    fn evaluator_config(&self) -> &EvaluatorConfig;
    fn vm_config(&self) -> &VMConfig {
        &self.evaluator_config().vm
    }

    fn evaluator<'a>(&'a self, sample_id: SampleId) -> Evaluator<'a> {
        todo!()
        // let dev = self.session().dev();
        // let sheet = &dev.sheets[sample_id.0];
        // let target_input = dev.load(sample_id).input;
        // Evaluator {
        //     sample_id,
        //     db: self.db(),
        //     target_input,
        //     sheet,
        //     evaluator_config: self.evaluator_config(),
        //     opt_static_husky_feature_eval: self.opt_static_husky_feature_eval(),
        // }
    }

    // None for 'static is shorter than 'static
    // Some(self) otherwise
    fn opt_static_husky_feature_eval(&self) -> Option<&dyn Runtime>;

    fn visualize_feature(&self, this: ValRepr, sample_id: SampleId) -> VMResult<VisualData> {
        self.evaluator(sample_id).eval_visual(this)
    }

    fn eval_feature_repr(&self, repr: &ValRepr, sample_id: SampleId) -> VMResult<RegularValue> {
        self.evaluator(sample_id).eval_feature_repr(repr)
    }

    fn eval_feature_repr_cached(
        &self,
        repr: &ValRepr,
        sample_id: SampleId,
    ) -> VMResult<RegularValue> {
        self.evaluator(sample_id).eval_feature_repr_cached(repr)
    }

    fn eval_feature_lazy_block(
        &self,
        block: &ValBlock,
        sample_id: SampleId,
    ) -> VMResult<RegularValue> {
        self.evaluator(sample_id).eval_lazy_block(block)
    }

    fn eval_feature_stmt(&self, stmt: &ValStmt, sample_id: SampleId) -> VMResult<RegularValue> {
        self.evaluator(sample_id).eval_stmt(stmt)
    }

    fn eval_feature_lazy_branch(
        &self,
        branch: ValBranch,
        sample_id: SampleId,
    ) -> VMResult<RegularValue> {
        self.evaluator(sample_id).eval_val_branch(branch)
    }

    fn eval_feature_expr(&self, expr: ValExpr, sample_id: SampleId) -> VMResult<RegularValue> {
        self.evaluator(sample_id).eval_expr(expr)
    }

    fn eval_feature_expr_cached(
        &self,
        expr: ValExpr,
        sample_id: SampleId,
    ) -> VMResult<RegularValue> {
        self.evaluator(sample_id).eval_expr_cached(expr)
    }

    fn eval_opt_domain_indicator_cached(
        &self,
        opt_arrival_indicator: Option<&ValDomain>,
        sample_id: SampleId,
    ) -> VMResult<bool> {
        self.evaluator(sample_id)
            .eval_opt_domain_indicator_cached(opt_arrival_indicator)
    }
}
