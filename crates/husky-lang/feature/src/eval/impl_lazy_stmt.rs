use vm::{EvalResult, EvalValue, VMError, VMResult};

use crate::*;

use super::FeatureEvaluator;

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    pub(super) fn eval_lazy_stmt(&mut self, stmt: &LazyStmt) -> EvalResult<'eval> {
        match stmt.kind {
            LazyStmtKind::Init { .. } => Ok(EvalValue::Undefined),
            LazyStmtKind::Assert { ref condition } => {
                if self.satisfies(condition)? {
                    Ok(EvalValue::Undefined)
                } else {
                    Err(VMError::AssertionFailure)
                }
            }
            LazyStmtKind::Return { ref result } => self.eval_lazy_expr(result),
            LazyStmtKind::Branches { ref branches, .. } => {
                for branch in branches {
                    let execute_branch: bool = match branch.kind {
                        LazyBranchKind::If { ref condition } => self.satisfies(condition)?,
                        LazyBranchKind::Elif { ref condition } => self.satisfies(condition)?,
                        LazyBranchKind::Else => true,
                    };
                    if execute_branch {
                        return self.eval_lazy_block(&branch.block);
                    }
                }
                Ok(EvalValue::Undefined)
            }
        }
    }

    fn satisfies(&mut self, condition: &LazyExpr) -> VMResult<bool> {
        Ok(match self.eval_lazy_expr(condition)? {
            EvalValue::Primitive(value) => match value {
                PrimitiveValue::I32(_) => todo!(),
                PrimitiveValue::F32(_) => todo!(),
                PrimitiveValue::B32(_) => todo!(),
                PrimitiveValue::B64(_) => todo!(),
                PrimitiveValue::Bool(b) => b,
                PrimitiveValue::Void => todo!(),
            },
            _ => todo!(),
        })
    }
}
