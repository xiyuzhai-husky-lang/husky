use vm::{EvalError, __Register, __VMError, __VMResult};

use crate::*;

use super::FeatureEvaluator;

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    pub(crate) fn eval_stmt(&mut self, stmt: &FeatureStmt) -> __VMResult<__Register<'eval>> {
        match stmt.variant {
            FeatureLazyStmtVariant::Init { .. } => Ok(__Register::new_unreturned()),
            FeatureLazyStmtVariant::Assert { ref condition } => {
                if self.satisfies(condition)? {
                    Ok(__Register::new_unreturned())
                } else {
                    Err(__VMError::new_normal(format!("assertion failed")))
                }
            }
            FeatureLazyStmtVariant::Return { ref result } => self.eval_expr(result),
            FeatureLazyStmtVariant::ReturnXml { ref result } => self.eval_xml_expr(result),
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
                Ok(__Register::new_unreturned())
            }
        }
    }

    fn satisfies(&mut self, condition: &FeatureExpr) -> __VMResult<bool> {
        Ok(self.eval_expr(condition)?.to_bool())
    }
}
