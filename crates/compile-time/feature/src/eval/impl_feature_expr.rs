use std::sync::Arc;

use semantics_lazy::LazyStmt;
use vm::{eval_fast, EvalResult, EvalValue, InstructionSheet, StackValue};

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
            FeatureExprKind::MembFuncCall {
                memb_ident,
                ref opds,
                ref stmts,
                ref instruction_sheet,
                compiled,
            } => self.eval_memb_routine_call(instruction_sheet, compiled, opds),
            FeatureExprKind::MembProcCall {
                memb_ident,
                ref opds,
                ref stmts,
                ref instruction_sheet,
                compiled,
            } => self.eval_memb_routine_call(instruction_sheet, compiled, opds),
            FeatureExprKind::MembPattCall {
                memb_ident,
                ref opds,
                ref instruction_sheet,
                ref stmts,
            } => todo!(),
            FeatureExprKind::ScopedFeature { ref stmts, .. } => self.eval_scoped_feature(stmts),
        }
    }

    fn eval_memb_routine_call(
        &mut self,
        instrns: &InstructionSheet,
        maybe_compiled: Option<()>,
        opds: &[Arc<FeatureExpr>],
    ) -> EvalResult<'eval> {
        let values = opds
            .iter()
            .map(|expr| StackValue::from_eval(self.eval_feature_expr(expr)?));
        eval_fast(values, instrns, maybe_compiled)
    }

    fn eval_routine_call(
        &mut self,
        instrns: &InstructionSheet,
        maybe_compiled: Option<()>,
        inputs: &[Arc<FeatureExpr>],
    ) -> EvalResult<'eval> {
        let values = inputs
            .iter()
            .map(|expr| StackValue::from_eval(self.eval_feature_expr(expr)?));
        eval_fast(values, instrns, maybe_compiled)
    }

    fn eval_scoped_feature(&mut self, stmts: &[Arc<LazyStmt>]) -> EvalResult<'eval> {
        todo!()
    }
}
