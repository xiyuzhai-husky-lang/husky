use vm::{EvalResult, EvalValue};

use crate::{FeatureExpr, FeatureExprKind};

use super::FeatureEvaluator;

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    pub(super) fn eval_feature_expr(&mut self, expr: &FeatureExpr) -> EvalResult<'eval> {
        match expr.kind {
            FeatureExprKind::PrimitiveLiteral(value) => Ok(value.into()),
            FeatureExprKind::EnumLiteral { ref value, uid } => {
                Ok(EvalValue::Boxed(value.clone_any()))
            }
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
                .cache(expr.feature, |evaluator: &mut Self| {
                    evaluator.eval_feature_expr(&value)
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
            FeatureExprKind::MembVarAccess {
                ref this,
                memb_var_ident,
                contract,
                opt_compiled,
            } => {
                if let Some(compiled) = opt_compiled {
                    todo!()
                } else {
                    let this_value = self.eval_feature_expr(this)?;
                    Ok(unsafe { this_value.lazy_memb_var(memb_var_ident, contract) })
                }
            }
        }
    }
}
