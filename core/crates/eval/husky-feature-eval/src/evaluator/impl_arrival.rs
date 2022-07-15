use crate::*;
use husky_entity_route::EntityRoutePtr;
use husky_entity_semantics::{CallFormSource, EntityDefnVariant};
use husky_feature_gen::*;
use husky_lazy_semantics::LazyStmt;
use husky_trace_protocol::VisualData;
use print_utils::{epin, msg_once, p};
use std::{iter::zip, panic::catch_unwind, sync::Arc};
use vm::__Linkage;
use vm::*;
use word::IdentPairDict;

use super::FeatureEvaluator;

impl<'temp, 'eval: 'temp> FeatureEvaluator<'temp, 'eval> {
    pub(crate) fn eval_opt_arrival_indicator_cached(
        &mut self,
        opt_arrival_indicator: Option<&Arc<FeatureArrivalIndicator>>,
    ) -> __EvalResult<bool> {
        if let Some(arrival_indicator) = opt_arrival_indicator {
            self.eval_cached(EvalKey::Feature(arrival_indicator.feature), |this| {
                Ok(__EvalValue::Copyable(
                    this.eval_arrival_indicator(arrival_indicator)?.into(),
                ))
            })
            .map(|v| v.primitive().take_bool())
        } else {
            Ok(true)
        }
    }
    fn eval_arrival_indicator(
        &mut self,
        arrival_indicator: &Arc<FeatureArrivalIndicator>,
    ) -> __EvalResult<bool> {
        Ok(match arrival_indicator.variant {
            FeatureBranchIndicatorVariant::AfterStmtNotReturn { ref stmt } => {
                if !self.eval_opt_arrival_indicator_cached(stmt.opt_arrival_indicator.as_ref())? {
                    return Ok(false);
                }
                self.eval_feature_stmt(stmt)? == __EvalValue::Unreturned
            }
            FeatureBranchIndicatorVariant::AfterConditionNotMet {
                ref opt_parent,
                ref condition,
            } => {
                if !self.eval_opt_arrival_indicator_cached(opt_parent.as_ref())? {
                    return Ok(false);
                }
                !self.eval_feature_expr(condition)?.primitive().take_bool()
            }
            FeatureBranchIndicatorVariant::IfConditionMet {
                ref opt_parent,
                ref condition,
            } => {
                if !self.eval_opt_arrival_indicator_cached(opt_parent.as_ref())? {
                    return Ok(false);
                }
                self.eval_feature_expr(condition)?.primitive().take_bool()
            }
        })
    }
}
