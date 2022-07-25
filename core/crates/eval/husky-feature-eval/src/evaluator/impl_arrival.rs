use crate::*;
use husky_entity_route::EntityRoutePtr;
use husky_entity_semantics::{CallFormSource, EntityDefnVariant};
use husky_feature_gen::*;
use husky_lazy_semantics::LazyStmt;
use husky_print_utils::{epin, msg_once, p};
use husky_trace_protocol::VisualData;
use husky_word::IdentPairDict;
use std::{iter::zip, panic::catch_unwind, sync::Arc};
use vm::__Linkage;
use vm::*;

use super::FeatureEvaluator;

impl<'temp, 'eval: 'temp> FeatureEvaluator<'temp, 'eval> {
    pub(crate) fn eval_opt_arrival_indicator_cached(
        &mut self,
        opt_arrival_indicator: Option<&Arc<FeatureArrivalIndicator>>,
    ) -> __VMResult<bool> {
        if let Some(arrival_indicator) = opt_arrival_indicator {
            self.eval_cached(EvalKey::Feature(arrival_indicator.feature), |this| {
                Ok(this
                    .eval_arrival_indicator(arrival_indicator)?
                    .to_register())
            })
            .map(|v| v.primitive().take_bool())
        } else {
            Ok(true)
        }
    }

    fn eval_arrival_indicator(
        &mut self,
        arrival_indicator: &Arc<FeatureArrivalIndicator>,
    ) -> __VMResult<bool> {
        Ok(match arrival_indicator.variant {
            FeatureBranchIndicatorVariant::AfterStmtNotReturn { ref stmt } => {
                if !self.eval_opt_arrival_indicator_cached(stmt.opt_arrival_indicator.as_ref())? {
                    return Ok(false);
                }
                self.eval_stmt(stmt)? == __Register::new_unreturned()
            }
            FeatureBranchIndicatorVariant::AfterConditionNotMet {
                ref opt_parent,
                ref condition,
            } => {
                if !self.eval_opt_arrival_indicator_cached(opt_parent.as_ref())? {
                    return Ok(false);
                }
                !self.eval_expr(condition)?.primitive().take_bool()
            }
            FeatureBranchIndicatorVariant::IfConditionMet {
                ref opt_parent,
                ref condition,
            } => {
                if !self.eval_opt_arrival_indicator_cached(opt_parent.as_ref())? {
                    return Ok(false);
                }
                self.eval_expr(condition)?.primitive().take_bool()
            }
        })
    }
}
