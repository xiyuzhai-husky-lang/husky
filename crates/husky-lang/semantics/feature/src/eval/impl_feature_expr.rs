use vm::EvalResult;

use crate::{FeatureExpr, FeatureExprKind};

use super::FeatureEvaluator;

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    pub(super) fn eval_feature_expr(&mut self, expr: &FeatureExpr) -> EvalResult<'eval> {
        match expr.kind {
            FeatureExprKind::Literal(value) => Ok(value.into()),
            FeatureExprKind::PrimitiveBinaryOpr {
                opr,
                ref lopd,
                ref ropd,
            } => Ok(opr
                .act_on_primitives(
                    self.eval_feature_expr(lopd)?.as_primitive()?,
                    self.eval_feature_expr(ropd)?.as_primitive()?,
                )?
                .into()),
            FeatureExprKind::Variable { ref value, .. } => self
                .cache(expr.feature, |this: &mut Self| {
                    this.eval_feature_expr(&value)
                }),
            FeatureExprKind::FuncCall {
                ref instruction_sheet,
                compiled,
                ref inputs,
                ..
            } => self.eval_routine_call(instruction_sheet, compiled, inputs),
            FeatureExprKind::ProcCall {
                ref instruction_sheet,
                compiled,
                ref inputs,
                ..
            } => self.eval_routine_call(instruction_sheet, compiled, inputs),
        }
    }
}
