use husky_print_utils::p;
use husky_word::RootIdentifier;
use vm::{__Register, __VMError, __VMResult};

use crate::*;

use super::FeatureEvaluator;

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    pub(crate) fn eval_stmt(&self, stmt: &FeatureLazyStmt) -> __VMResult<__Register<'eval>> {
        match stmt.variant {
            FeatureLazyStmtVariant::Init { .. } => Ok(__Register::new_unreturned()),
            FeatureLazyStmtVariant::Assert { ref condition } => {
                if self.satisfies(condition)? {
                    Ok(__Register::new_unreturned())
                } else {
                    Err(__VMError::new_normal(format!(
                        "assertion failed at {:?}:{:?}",
                        stmt.file, stmt.range.start.row
                    )))
                }
            }
            FeatureLazyStmtVariant::Require { ref condition, .. } => {
                if self.satisfies(condition)? {
                    Ok(__Register::new_unreturned())
                } else {
                    Ok(__Register::new_none())
                }
            }
            FeatureLazyStmtVariant::Return { ref result } => self.eval_expr(result),
            FeatureLazyStmtVariant::ReturnXml { ref result } => self.eval_xml_expr(result),
            FeatureLazyStmtVariant::ConditionFlow { ref branches, .. } => {
                for branch in branches {
                    let execute_branch: bool = match branch.variant {
                        FeatureLazyBranchVariant::If { ref condition } => {
                            self.satisfies(condition)?
                        }
                        FeatureLazyBranchVariant::Elif { ref condition } => {
                            self.satisfies(condition)?
                        }
                        FeatureLazyBranchVariant::Else => true,
                    };
                    if execute_branch {
                        return self.eval_feature_lazy_block(&branch.block);
                    }
                }
                Ok(__Register::new_unreturned())
            }
        }
    }

    fn satisfies(&self, condition: &FeatureLazyExpr) -> __VMResult<bool> {
        let value = self.eval_expr(condition)?;
        let value_str = self
            .db
            .comptime()
            .print_short(&value, RootIdentifier::Bool.into());
        Ok(value.to_bool())
    }
}
