use vm::{Conditional, EvalValue};

use crate::{FeatureExpr, FeatureExprKind};

use super::FeatureEvaluator;

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    pub(super) fn eval_feature_expr(&mut self, expr: &FeatureExpr) -> EvalValue<'eval, 'eval> {
        self.indicator.set(expr.eval_id);
        match expr.kind {
            FeatureExprKind::Literal(value) => Ok(Conditional::Defined(value.into())),
            FeatureExprKind::PrimitiveBinaryOpr {
                opr,
                ref lopd,
                ref ropd,
            } => Ok(Conditional::Defined(
                opr.act_on_primitives(
                    self.eval_feature_expr(lopd)?
                        .defined_ref()?
                        .as_primitive()?,
                    self.eval_feature_expr(ropd)?
                        .defined_ref()?
                        .as_primitive()?,
                )?
                .into(),
            )),
            FeatureExprKind::Variable { ref value, .. } => self
                .cache(expr.feature, |this: &mut Self| {
                    this.eval_feature_expr(&value)
                }),
            FeatureExprKind::FuncCall {
                ref instructions,
                compiled,
                ref inputs,
                ..
            } => self.eval_func_call(instructions, compiled, inputs),
        }
    }
}
