use vm::{Conditional, EvalValue, StackValue, VMError, VMResult};

use crate::*;

use super::FeatureEvaluator;

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    pub(super) fn eval_feature_stmt(&mut self, stmt: &FeatureStmt) -> EvalValue<'eval, 'eval> {
        self.indicator.set(stmt.eval_id);
        match stmt.kind {
            FeatureStmtKind::Init { .. } => Ok(Conditional::Undefined),
            FeatureStmtKind::Assert { ref condition } => {
                if self.satisfies(condition)? {
                    Ok(Conditional::Undefined)
                } else {
                    Err(VMError::AssertionFailure)
                }
            }
            FeatureStmtKind::Return { ref result } => self.eval_feature_expr(result),
            FeatureStmtKind::Branches { ref branches, .. } => {
                for branch in branches {
                    let execute_branch: bool = match branch.kind {
                        FeatureBranchKind::If { ref condition } => self.satisfies(condition)?,
                        FeatureBranchKind::Elif { ref condition } => self.satisfies(condition)?,
                        FeatureBranchKind::Else => true,
                    };
                    if execute_branch {
                        return self.eval_feature_block(&branch.block);
                    }
                }
                Ok(Conditional::Undefined)
            }
        }
    }

    fn satisfies(&mut self, condition: &FeatureExpr) -> VMResult<bool> {
        Ok(match self.eval_feature_expr(condition)?.defined_ref()? {
            StackValue::Primitive(value) => match value {
                PrimitiveValue::I32(_) => todo!(),
                PrimitiveValue::F32(_) => todo!(),
                PrimitiveValue::B32(_) => todo!(),
                PrimitiveValue::B64(_) => todo!(),
                PrimitiveValue::Bool(b) => *b,
                PrimitiveValue::Void => todo!(),
            },
            _ => todo!(),
        })
    }
}
