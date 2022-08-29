mod evaluator;
mod session;

pub use evaluator::*;
use husky_entity_route::EntityRoutePtr;
use husky_trace_protocol::{SampleId, VisualData};
pub use session::*;

use husky_feature_gen::*;
use husky_vm::{VMConfig, __Register, __RegisterDataKind, __VMResult};
use std::sync::{Arc, Mutex};
use upcast::Upcast;

pub trait EvalFeature<'eval>: FeatureGenQueryGroup + Upcast<dyn FeatureGenQueryGroup> {
    fn session(&self) -> &Session<'eval>;
    fn evaluator_config(&self) -> &EvaluatorConfig;
    fn vm_config(&self) -> &VMConfig {
        &self.evaluator_config().vm
    }

    fn evaluator<'a>(&'a self, sample_id: SampleId) -> FeatureEvaluator<'a, 'eval> {
        let dev = self.session().dev();
        let sheet = &dev.sheets[sample_id.0];
        let target_input = dev.load(sample_id).input;
        FeatureEvaluator {
            sample_id,
            db: self.upcast(),
            target_input,
            sheet,
            evaluator_config: self.evaluator_config(),
            opt_static_husky_feature_eval: self.opt_static_husky_feature_eval(),
        }
    }

    // None for 'eval is shorter than 'static
    // Some(self) otherwise
    fn opt_static_husky_feature_eval(&self) -> Option<&dyn EvalFeature<'static>>;

    fn visualize_feature(&self, this: FeatureRepr, sample_id: SampleId) -> __VMResult<VisualData>
    where
        'eval: 'static,
    {
        self.evaluator(sample_id).visualize_feature(this)
    }

    fn eval_feature_repr(
        &self,
        repr: &FeatureRepr,
        sample_id: SampleId,
    ) -> __VMResult<__Register<'eval>> {
        self.evaluator(sample_id).eval_feature_repr(repr)
    }

    fn eval_feature_repr_cached(
        &self,
        repr: &FeatureRepr,
        sample_id: SampleId,
    ) -> __VMResult<__Register<'eval>> {
        self.evaluator(sample_id).eval_feature_repr_cached(repr)
    }

    fn eval_feature_lazy_block(
        &self,
        block: &FeatureLazyBlock,
        sample_id: SampleId,
    ) -> __VMResult<__Register<'eval>> {
        self.evaluator(sample_id).eval_lazy_block(block)
    }

    fn eval_feature_stmt(
        &self,
        stmt: &FeatureLazyStmt,
        sample_id: SampleId,
    ) -> __VMResult<__Register<'eval>> {
        self.evaluator(sample_id).eval_stmt(stmt)
    }

    fn eval_feature_lazy_branch(
        &self,
        branch: &FeatureLazyBranch,
        sample_id: SampleId,
    ) -> __VMResult<__Register<'eval>> {
        self.evaluator(sample_id).eval_lazy_branch(branch)
    }

    fn eval_feature_expr(
        &self,
        expr: &FeatureLazyExpr,
        sample_id: SampleId,
    ) -> __VMResult<__Register<'eval>> {
        self.evaluator(sample_id).eval_expr(expr)
    }

    fn eval_opt_arrival_indicator(
        &self,
        opt_arrival_indicator: Option<&Arc<FeatureArrivalIndicator>>,
        sample_id: SampleId,
    ) -> __VMResult<bool> {
        Ok(if let Some(ref arrival_indicator) = opt_arrival_indicator {
            match arrival_indicator.variant {
                FeatureBranchIndicatorVariant::AfterStmtNotReturn { ref stmt } => {
                    if !self.eval_opt_arrival_indicator(
                        stmt.opt_arrival_indicator.as_ref(),
                        sample_id,
                    )? {
                        return Ok(false);
                    }
                    self.eval_feature_stmt(stmt, sample_id)?.data_kind()
                        == __RegisterDataKind::Unreturned
                }
                FeatureBranchIndicatorVariant::AfterConditionNotMet {
                    ref opt_parent,
                    ref condition,
                } => {
                    if !self.eval_opt_arrival_indicator(opt_parent.as_ref(), sample_id)? {
                        return Ok(false);
                    }
                    !self
                        .eval_feature_expr(condition, sample_id)?
                        .downcast_bool()
                }
                FeatureBranchIndicatorVariant::IfConditionMet {
                    ref opt_parent,
                    ref condition,
                } => {
                    if !self.eval_opt_arrival_indicator(opt_parent.as_ref(), sample_id)? {
                        return Ok(false);
                    }
                    self.eval_feature_expr(condition, sample_id)?
                        .downcast_bool()
                }
            }
        } else {
            true
        })
    }
}
