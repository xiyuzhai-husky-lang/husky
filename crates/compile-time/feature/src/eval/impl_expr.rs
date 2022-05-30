use crate::*;
use print_utils::{epin, p};
use semantics_lazy::LazyStmt;
use std::sync::Arc;
use vm::*;

use super::FeatureEvaluator;

impl<'vm, 'eval: 'vm> FeatureEvaluator<'vm, 'eval> {
    pub(super) fn eval_feature_expr(&mut self, expr: &FeatureExpr) -> EvalResult<'eval> {
        match expr.variant {
            FeatureExprVariant::PrimitiveLiteral(value) => Ok(value.into()),
            FeatureExprVariant::EnumKindLiteral { entity_route, uid } => {
                todo!()
                // Ok(EvalValue::Boxed(value.clone_any()))
            }
            FeatureExprVariant::PrimitiveBinaryOpr {
                opr,
                ref lopd,
                ref ropd,
            } => Ok(opr
                .act_on_primitives(
                    self.eval_feature_expr(lopd)?.primitive(),
                    self.eval_feature_expr(ropd)?.primitive(),
                )?
                .into()),
            FeatureExprVariant::StructOriginalFieldAccess {
                ref this,
                field_idx,
                field_binding,
                opt_linkage: opt_compiled,
                ..
            } => {
                if let Some(compiled) = opt_compiled {
                    todo!()
                } else {
                    let this_value = self.eval_feature_expr(this)?;
                    Ok(unsafe { this_value.lazy_field(field_idx, field_binding) })
                }
            }
            FeatureExprVariant::RoutineCall {
                ref opds,
                ref opt_instruction_sheet,
                opt_linkage,
                has_this,
                ..
            } => {
                let result = self.eval_routine_call(
                    opt_instruction_sheet.as_ref().map(|r| &**r),
                    opt_linkage,
                    opds,
                    has_this,
                );
                result
            }
            FeatureExprVariant::EntityFeature {
                repr: ref block, ..
            } => todo!(),
            // self.eval_feature_block(block, EvalKey::Feature(block.feature)),
            FeatureExprVariant::NewRecord {
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
            FeatureExprVariant::Variable { ref value, .. } => self
                .cache(EvalKey::Feature(expr.feature), |evaluator: &mut Self| {
                    evaluator.eval_feature_expr(&value)
                }),
            FeatureExprVariant::RecordOriginalFieldAccess {
                ref this,
                field_ident,
                ref repr,
            } => self.eval_feature_repr(repr, EvalKey::Feature(repr.feature())),
            FeatureExprVariant::This { ref repr } => todo!(),
            FeatureExprVariant::GlobalInput => Ok(EvalValue::GlobalPure(self.eval_input.clone())),
            FeatureExprVariant::PatternCall {} => todo!(),
            FeatureExprVariant::RecordDerivedFieldAccess {
                ref this,
                field_ident,
                ref repr,
                ..
            } => self.eval_feature_repr(repr, EvalKey::Feature(repr.feature())),
            FeatureExprVariant::ElementAccess {
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
            FeatureExprVariant::StructDerivedFieldAccess {
                ref this,
                field_ident,
                ref repr,
            } => todo!(),
        }
    }

    fn eval_routine_call(
        &mut self,
        opt_instrns: Option<&InstructionSheet>,
        maybe_compiled: Option<Linkage>,
        arguments: &[Arc<FeatureExpr>],
        has_this: bool,
    ) -> EvalResult<'eval> {
        let db = self.db;
        let values = arguments
            .iter()
            .map(|expr| VMValue::from_eval(self.eval_feature_expr(expr)?));
        eval_fast(db.upcast(), opt_instrns, maybe_compiled, values)
    }
}
