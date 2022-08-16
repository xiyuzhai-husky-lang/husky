use husky_print_utils::p;
use husky_word::RootIdentifier;
use vm::{__Register, __VMError, __VMResult};

use crate::*;

use super::FeatureEvaluator;

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    pub(crate) fn eval_stmt(&self, stmt: &FeatureStmt) -> __VMResult<__Register<'eval>> {
        match stmt.variant {
            FeatureStmtVariant::Init { .. } => Ok(__Register::new_unreturned()),
            FeatureStmtVariant::Assert { ref condition } => {
                if self.satisfies(condition)? {
                    Ok(__Register::new_unreturned())
                } else {
                    Err(__VMError::new_normal(format!(
                        "assertion failed at {:?}:{:?}",
                        stmt.file, stmt.range.start.row
                    )))
                }
            }
            FeatureStmtVariant::Require { ref condition, .. } => {
                if self.satisfies(condition)? {
                    Ok(__Register::new_unreturned())
                } else {
                    Ok(__Register::new_undefined())
                }
            }
            FeatureStmtVariant::Return { ref result } => self.eval_expr(result),
            FeatureStmtVariant::ReturnXml { ref result } => self.eval_xml_expr(result),
            FeatureStmtVariant::ConditionFlow { ref branches, .. } => {
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

    fn satisfies(&self, condition: &FeatureExpr) -> __VMResult<bool> {
        let value = self.eval_expr(condition)?;
        let value_str = self
            .db
            .comptime()
            .print_short(&value, RootIdentifier::Bool.into());
        Ok(value.to_bool())
    }
}
