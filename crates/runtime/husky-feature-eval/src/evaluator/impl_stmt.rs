use husky_opn_semantics::ImplicitConversion;

use husky_vm::{__Register, __VMError, __VMResult};

use crate::*;

use super::FeatureEvaluator;

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    pub(crate) fn eval_stmt(&self, stmt: &FeatureLazyStmt) -> __VMResult<__Register<'eval>> {
        match stmt.variant {
            FeatureLazyStmtVariant::Init { .. } => Ok(__Register::unreturned()),
            FeatureLazyStmtVariant::Assert { ref condition } => {
                if self.satisfies(condition)? {
                    Ok(__Register::unreturned())
                } else {
                    Err(__VMError::new_normal(format!(
                        "assertion failed at {:?}:{:?}",
                        stmt.file, stmt.range.start.row
                    )))
                }
            }
            FeatureLazyStmtVariant::Require { ref condition, .. } => {
                if self.satisfies(condition)? {
                    Ok(__Register::unreturned())
                } else {
                    Ok(__Register::none(0))
                }
            }
            FeatureLazyStmtVariant::Return { ref result } => self.eval_expr(result),
            FeatureLazyStmtVariant::ReturnUnveil {
                ref result,
                implicit_conversion,
                ..
            } => self.eval_expr(result).map(|v| {
                v.unveil()
                    .map(|v| match implicit_conversion {
                        ImplicitConversion::None => v,
                        ImplicitConversion::WrapInSome { number_of_somes } => {
                            v.wrap_in_some(number_of_somes)
                        }
                        ImplicitConversion::ConvertToBool => todo!(),
                    })
                    .unwrap_or(__Register::unreturned())
            }),
            FeatureLazyStmtVariant::ReturnHtml { ref result } => self.eval_xml_expr(result),
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
                        return self.eval_lazy_block(&branch.block);
                    }
                }
                Ok(__Register::unreturned())
            }
        }
    }

    fn satisfies(&self, condition: &FeatureLazyExpr) -> __VMResult<bool> {
        let value = self.eval_expr_cached(condition)?;
        Ok(value.to_bool())
    }
}
