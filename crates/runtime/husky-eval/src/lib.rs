pub mod db;
// mod evaluator;
// mod session;

// pub use self::evaluator::*;
// pub use self::session::*;
use husky_ethereal_term::EtherealTerm;
use husky_trace_protocol::{SampleId, VisualData};

use husky_val_repr::*;
use husky_vm::{VMConfig, __Register, __VMResult};
use std::sync::{Arc, Mutex};
use upcast::Upcast;

// pub trait EvalFeature<'eval>: ValReprDb {
//     fn session(&self) -> &Session<'eval>;
//     fn evaluator_config(&self) -> &EvaluatorConfig;
//     fn vm_config(&self) -> &VMConfig {
//         &self.evaluator_config().vm
//     }

//     fn evaluator<'a>(&'a self, sample_id: SampleId) -> FeatureEvaluator<'a, 'eval> {
//         let dev = self.session().dev();
//         let sheet = &dev.sheets[sample_id.0];
//         let target_input = dev.load(sample_id).input;
//         FeatureEvaluator {
//             sample_id,
//             db: self.upcast(),
//             target_input,
//             sheet,
//             evaluator_config: self.evaluator_config(),
//             opt_static_husky_feature_eval: self.opt_static_husky_feature_eval(),
//         }
//     }

//     // None for 'eval is shorter than 'static
//     // Some(self) otherwise
//     fn opt_static_husky_feature_eval(&self) -> Option<&dyn EvalFeature<'static>>;

//     fn visualize_feature(&self, this: ValRepr, sample_id: SampleId) -> __VMResult<VisualData> {
//         self.evaluator(sample_id).visualize_feature(this)
//     }

//     fn eval_feature_repr(
//         &self,
//         repr: &ValRepr,
//         sample_id: SampleId,
//     ) -> __VMResult<__Register<'eval>> {
//         self.evaluator(sample_id).eval_feature_repr(repr)
//     }

//     fn eval_feature_repr_cached(
//         &self,
//         repr: &ValRepr,
//         sample_id: SampleId,
//     ) -> __VMResult<__Register<'eval>> {
//         self.evaluator(sample_id).eval_feature_repr_cached(repr)
//     }

//     fn eval_feature_lazy_block(
//         &self,
//         block: &ValBlock,
//         sample_id: SampleId,
//     ) -> __VMResult<__Register<'eval>> {
//         self.evaluator(sample_id).eval_lazy_block(block)
//     }

//     fn eval_feature_stmt(
//         &self,
//         stmt: &ValStmt,
//         sample_id: SampleId,
//     ) -> __VMResult<__Register<'eval>> {
//         self.evaluator(sample_id).eval_stmt(stmt)
//     }

//     fn eval_feature_lazy_branch(
//         &self,
//         branch: &FeatureLazyBranch,
//         sample_id: SampleId,
//     ) -> __VMResult<__Register<'eval>> {
//         self.evaluator(sample_id).eval_lazy_branch(branch)
//     }

//     fn eval_feature_expr(
//         &self,
//         expr: &FeatureLazyExpr,
//         sample_id: SampleId,
//     ) -> __VMResult<__Register<'eval>> {
//         self.evaluator(sample_id).eval_expr(expr)
//     }

//     fn eval_feature_expr_cached(
//         &self,
//         expr: &FeatureLazyExpr,
//         sample_id: SampleId,
//     ) -> __VMResult<__Register<'eval>> {
//         self.evaluator(sample_id).eval_expr_cached(expr)
//     }

//     fn eval_opt_domain_indicator_cached(
//         &self,
//         opt_arrival_indicator: Option<&ValDomain>,
//         sample_id: SampleId,
//     ) -> __VMResult<bool> {
//         self.evaluator(sample_id)
//             .eval_opt_domain_indicator_cached(opt_arrival_indicator)
//     }
// }
