use vm::{EvalError, __EvalResult, __EvalValue};

use crate::*;

use super::FeatureEvaluator;

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    pub(crate) fn husky_feature_eval_stmt(
        &mut self,
        stmt: &FeatureStmt,
    ) -> __EvalValueResult<'eval> {
        match stmt.variant {
            FeatureLazyStmtVariant::Init { .. } => Ok(__EvalValue::Unreturned),
            FeatureLazyStmtVariant::Assert { ref condition } => {
                if self.satisfies(condition)? {
                    Ok(__EvalValue::Unreturned)
                } else {
                    Err(EvalError::Normal {
                        message: format!("assertion failed"),
                    }
                    .into())
                }
            }
            FeatureLazyStmtVariant::Return { ref result } => self.eval_feature_expr(result),
            FeatureLazyStmtVariant::ReturnXml { ref result } => {
                self.husky_feature_eval_xml_expr(result)
            }
            FeatureLazyStmtVariant::ConditionFlow { ref branches, .. } => {
                for branch in branches {
                    let execute_branch: bool = match branch.variant {
                        FeatureBranchVariant::If { ref condition } => self.satisfies(condition)?,
                        FeatureBranchVariant::Elif { ref condition } => {
                            self.satisfies(condition)?
                        }
                        FeatureBranchVariant::Else => true,
                    };
                    if execute_branch {
                        return self.husky_feature_eval_lazy_block(&branch.block);
                    }
                }
                Ok(__EvalValue::Undefined)
            }
        }
    }

    fn satisfies(&mut self, condition: &FeatureExpr) -> __EvalResult<bool> {
        Ok(self.eval_feature_expr(condition)?.primitive().to_bool())
    }
}
