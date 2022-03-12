use vm::EvalResult;

use crate::{LazyExpr, LazyExprKind};

use super::FeatureEvaluator;

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    pub(super) fn eval_lazy_expr(&mut self, expr: &LazyExpr) -> EvalResult<'eval> {
        match expr.kind {
            LazyExprKind::Literal(value) => Ok(value.into()),
            LazyExprKind::PrimitiveBinaryOpr {
                opr,
                ref lopd,
                ref ropd,
            } => Ok(opr
                .act_on_primitives(
                    self.eval_lazy_expr(lopd)?.as_primitive()?,
                    self.eval_lazy_expr(ropd)?.as_primitive()?,
                )?
                .into()),
            LazyExprKind::Variable { ref value, .. } => {
                self.cache(expr.feature, |this: &mut Self| this.eval_lazy_expr(&value))
            }
            LazyExprKind::FuncCall {
                ref instruction_sheet,
                compiled,
                ref inputs,
                ..
            } => self.eval_routine_call(instruction_sheet, compiled, inputs),
            LazyExprKind::ProcCall {
                ref instruction_sheet,
                compiled,
                ref inputs,
                ..
            } => self.eval_routine_call(instruction_sheet, compiled, inputs),
        }
    }
}
