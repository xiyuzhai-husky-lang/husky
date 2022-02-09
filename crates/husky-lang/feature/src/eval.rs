use vm::{PrimitiveValue, VMError};

use crate::{expr::FeatureExprKind, sheet::FeatureSheet, stmt::FeatureStmtKind, *};
use vm::{Conditional, EvalValue, StackValue};

pub fn eval_feature_block<'eval>(
    input: EvalValue<'eval, 'eval>,
    sheet: &mut FeatureSheet<'eval>,
    block: &FeatureBlock,
) -> EvalValue<'eval, 'eval> {
    let mut evaluator = FeatureEvaluator { input, sheet };
    evaluator.eval_block(block)
}

pub fn eval_feature_stmt<'eval>(
    input: EvalValue<'eval, 'eval>,
    sheet: &mut FeatureSheet<'eval>,
    stmt: &FeatureStmt,
) -> EvalValue<'eval, 'eval> {
    let mut evaluator = FeatureEvaluator { input, sheet };
    evaluator.eval_stmt(stmt)
}

pub fn eval_feature_expr<'eval>(
    input: EvalValue<'eval, 'eval>,
    sheet: &mut FeatureSheet<'eval>,
    expr: &FeatureExpr,
) -> EvalValue<'eval, 'eval> {
    let mut evaluator = FeatureEvaluator { input, sheet };
    evaluator.eval_expr(expr)
}

pub struct FeatureEvaluator<'a, 'eval: 'a> {
    input: EvalValue<'eval, 'eval>,
    sheet: &'a mut FeatureSheet<'eval>,
}

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    fn eval_expr(&mut self, expr: &FeatureExpr) -> EvalValue<'eval, 'eval> {
        match expr.kind {
            FeatureExprKind::Literal(value) => Ok(Conditional::Defined(value.into())),
            FeatureExprKind::PrimitiveBinaryOpr {
                opr,
                ref lopd,
                ref ropd,
            } => Ok(Conditional::Defined(
                opr.act_on_primitives(
                    self.eval_expr(lopd)?.defined_ref()?.as_primitive()?,
                    self.eval_expr(ropd)?.defined_ref()?.as_primitive()?,
                )?
                .into(),
            )),
            FeatureExprKind::Variable { ref value, .. } => {
                self.cache(expr.feature, |this: &mut Self| this.eval_expr(&value))
            }
        }
        //     match *cached_ptr.ptr {
        //         Feature::Input => todo!(),
        //         Feature::Literal(literal) => Ok(Conditional::Defined(StackValue::Primitive(literal))),
        //         Feature::Assert { condition } => match self.eval(condition) {
        //             Ok(_) => Ok(Conditional::Undefined),
        //             Err(_) => todo!(),
        //         },
        //         Feature::Do { first, then } => match self.eval(first)? {
        //             Conditional::Defined(value) => Ok(Conditional::Defined(value)),
        //             Conditional::Undefined => self.eval(then),
        //         },
        //         Feature::PrimitiveBinaryFunc { func, lopd, ropd } => Ok(Conditional::Defined(
        //             StackValue::Primitive(func.act_on_primitives(
        //                 self.eval(lopd)?.defined()?.as_primitive()?,
        //                 self.eval(ropd)?.defined()?.as_primitive()?,
        //             )?),
        //         )),
        //     }
    }

    fn eval_stmt(&mut self, stmt: &FeatureStmt) -> EvalValue<'eval, 'eval> {
        match stmt.kind {
            FeatureStmtKind::Init { .. } => Ok(Conditional::Undefined),
            FeatureStmtKind::Assert { ref condition } => {
                let satisfied: bool = match self.eval_expr(condition)?.defined_ref()? {
                    StackValue::Primitive(value) => match value {
                        PrimitiveValue::I32(_) => todo!(),
                        PrimitiveValue::F32(_) => todo!(),
                        PrimitiveValue::B32(_) => todo!(),
                        PrimitiveValue::B64(_) => todo!(),
                        PrimitiveValue::Bool(b) => *b,
                        PrimitiveValue::Void => todo!(),
                    },
                    _ => todo!(),
                };
                if satisfied {
                    Ok(Conditional::Undefined)
                } else {
                    Err(VMError::AssertionFailure)
                }
            }
            FeatureStmtKind::Return { ref result } => self.eval_expr(result),
            FeatureStmtKind::Branches { .. } => todo!(),
        }
    }

    fn eval_block(&mut self, block: &FeatureBlock) -> EvalValue<'eval, 'eval> {
        self.cache(block.feature, |this: &mut Self| {
            for stmt in block.stmts.iter() {
                let value = this.eval_stmt(stmt)?;
                match value {
                    Conditional::Defined(_) => return Ok(value),
                    Conditional::Undefined => (),
                }
            }
            Ok(Conditional::Undefined)
        })
    }

    fn cache(
        &mut self,
        feature: FeaturePtr,
        compute_value: impl FnOnce(&mut Self) -> EvalValue<'eval, 'eval>,
    ) -> EvalValue<'eval, 'eval> {
        if let Some(value) = self.sheet.cached_value(feature) {
            value
        } else {
            let value = compute_value(self);
            self.sheet.cache(feature, value)
        }
    }
}
