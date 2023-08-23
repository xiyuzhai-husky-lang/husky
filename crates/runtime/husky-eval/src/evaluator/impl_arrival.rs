use crate::*;
use husky_val_repr::*;

use husky_vm::*;
use std::sync::Arc;

use super::FeatureEvaluator;

impl<'temp> FeatureEvaluator<'temp> {
    #[inline(always)]
    pub(crate) fn eval_opt_domain_indicator_cached(
        &self,
        opt_arrival_indicator: Option<&ValDomain>,
    ) -> __VMResult<bool> {
        if let Some(arrival_indicator) = opt_arrival_indicator {
            self.eval_cached(EvalKey::Feature(arrival_indicator.feature), |this| {
                Ok(this
                    .eval_arrival_indicator(arrival_indicator)?
                    .to_register())
            })
            .map(|v| v.downcast_bool())
        } else {
            Ok(true)
        }
    }

    fn eval_arrival_indicator(&self, arrival_indicator: &ValDomain) -> __VMResult<bool> {
        Ok(match arrival_indicator.variant {
            ValDomainData::AfterStmtNotReturn { ref stmt } => {
                if !self.eval_opt_domain_indicator_cached(stmt.opt_arrival_indicator.as_ref())? {
                    return Ok(false);
                }
                self.eval_stmt(stmt)?.data_kind() == __RegisterDataKind::Unreturned
            }
            ValDomainData::AfterConditionNotMet {
                ref opt_parent,
                ref condition,
            } => {
                if !self.eval_opt_domain_indicator_cached(opt_parent.as_ref())? {
                    return Ok(false);
                }
                !self.eval_expr(condition)?.downcast_bool()
            }
            ValDomainData::IfConditionMet {
                ref opt_parent,
                ref condition,
            } => {
                if !self.eval_opt_domain_indicator_cached(opt_parent.as_ref())? {
                    return Ok(false);
                }
                self.eval_expr(condition)?.downcast_bool()
            }
        })
    }
}
