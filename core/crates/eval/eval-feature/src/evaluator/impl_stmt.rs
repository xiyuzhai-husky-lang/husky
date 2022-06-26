use vm::{EvalError, EvalResult, EvalValue};

use crate::*;

use super::FeatureEvaluator;

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    pub(crate) fn eval_feature_stmt(
        &mut self,
        stmt: &FeatureLazyStmt,
    ) -> EvalResult<EvalValue<'eval>> {
        match stmt.variant {
            FeatureLazyStmtVariant::Init { .. } => Ok(EvalValue::Undefined),
            FeatureLazyStmtVariant::Assert { ref condition } => {
                if self.satisfies(condition)? {
                    Ok(EvalValue::Undefined)
                } else {
                    Err(EvalError::Normal {
                        message: format!("assertion failed"),
                    }
                    .into())
                }
            }
            FeatureLazyStmtVariant::Return { ref result } => self.eval_feature_lazy_expr(result),
            FeatureLazyStmtVariant::ReturnXml { ref result } => self.eval_feature_xml_expr(result),
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
                        return self.eval_feature_lazy_block(&branch.block);
                    }
                }
                Ok(EvalValue::Undefined)
            }
        }
    }

    fn satisfies(&mut self, condition: &FeatureLazyExpr) -> EvalResult<bool> {
        Ok(self
            .eval_feature_lazy_expr(condition)?
            .primitive()
            .to_bool())
    }
}
