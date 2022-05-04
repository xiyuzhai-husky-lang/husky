use std::sync::Arc;

use semantics_lazy::LazyStmt;
use vm::{eval_fast, EvalResult, EvalValue, InstructionSheet, Linkage, StackValue};

use crate::{FeatureBlock, FeatureExpr, FeatureExprKind};

use super::FeatureEvaluator;

impl<'stack, 'eval: 'stack> FeatureEvaluator<'stack, 'eval> {
    pub(super) fn eval_feature_expr(&mut self, expr: &FeatureExpr) -> EvalResult<'eval> {
        match expr.kind {
            FeatureExprKind::PrimitiveLiteral(value) => Ok(value.into()),
            FeatureExprKind::EnumLiteral { entity_route, uid } => {
                todo!()
                // Ok(EvalValue::Boxed(value.clone_any()))
            }
            FeatureExprKind::PrimitiveBinaryOpr {
                opr,
                ref lopd,
                ref ropd,
            } => Ok(opr
                .act_on_primitives(
                    self.eval_feature_expr(lopd)?.as_primitive(),
                    self.eval_feature_expr(ropd)?.as_primitive(),
                )?
                .into()),
            FeatureExprKind::StructOriginalFieldAccess {
                ref this,
                field_idx,
                contract,
                opt_linkage: opt_compiled,
                ..
            } => {
                if let Some(compiled) = opt_compiled {
                    todo!()
                } else {
                    let this_value = self.eval_feature_expr(this)?;
                    Ok(unsafe { this_value.lazy_field_var(field_idx, contract) })
                }
            }
            FeatureExprKind::RoutineCall {
                ref opds,
                ref opt_instruction_sheet,
                opt_linkage: opt_compiled,
                ..
            } => self.eval_routine_call(
                opt_instruction_sheet.as_ref().map(|r| &**r),
                opt_compiled,
                opds,
            ),
            FeatureExprKind::EntityFeature { ref block, .. } => self.eval_feature_block(block),
            FeatureExprKind::NewRecord {
                ty,
                ref entity,
                ref opds,
            } => {
                // Ok(self
                // .sheet
                // .resolve_class_call(self.db, expr.eval_id, entity, opds)
                // .into()),
                todo!()
            }
            FeatureExprKind::Variable { ref value, .. } => self
                .cache(expr.feature, |evaluator: &mut Self| {
                    evaluator.eval_feature_expr(&value)
                }),
            FeatureExprKind::RecordOriginalFieldAccess {
                ref this,
                field_ident,
                ref repr,
            } => self.eval_feature_repr(repr),
            FeatureExprKind::This { ref repr } => todo!(),
            FeatureExprKind::GlobalInput => Ok(EvalValue::GlobalPure(self.global_input.clone())),
            FeatureExprKind::PatternCall {} => todo!(),
            FeatureExprKind::RecordDerivedFieldAccess { ref block, .. } => {
                self.eval_feature_block(block)
            }
            FeatureExprKind::ElementAccess {
                ref opds, linkage, ..
            } => {
                if opds.len() > 2 {
                    todo!()
                }
                let mut values = vec![
                    self.eval_feature_expr(&opds[0])?.into_stack().unwrap(),
                    self.eval_feature_expr(&opds[1])?.into_stack().unwrap(),
                ];
                (linkage.call)(&mut values).map(|mut value| value.into_eval())
            }
        }
    }

    fn eval_routine_call(
        &mut self,
        opt_instrns: Option<&InstructionSheet>,
        maybe_compiled: Option<Linkage>,
        inputs: &[Arc<FeatureExpr>],
    ) -> EvalResult<'eval> {
        let db = self.db;
        let values = inputs
            .iter()
            .map(|expr| StackValue::from_eval(self.eval_feature_expr(expr)?));
        eval_fast(db.upcast(), values, opt_instrns, maybe_compiled)
    }
}
