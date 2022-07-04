use crate::*;
use husky_entity_route::EntityRoutePtr;
use husky_entity_semantics::{CallFormSource, EntityDefnVariant};
use husky_feature_gen::*;
use husky_lazy_semantics::LazyStmt;
use husky_trace_protocol::VisualData;
use print_utils::{epin, msg_once, p};
use std::{iter::zip, panic::catch_unwind, sync::Arc};
use vm::__Linkage;
use vm::*;
use word::IdentPairDict;

use super::FeatureEvaluator;

impl<'temp, 'eval: 'temp> FeatureEvaluator<'temp, 'eval> {
    pub(crate) fn husky_feature_eval_expr(&mut self, expr: &FeatureExpr) -> EvalValueResult<'eval> {
        match expr.variant {
            FeatureLazyExprVariant::PrimitiveLiteral(value) => Ok(value.into()),
            FeatureLazyExprVariant::EnumKindLiteral { entity_route, uid } => {
                todo!()
                // Ok(EvalValue::Boxed(value.clone_any()))
            }
            FeatureLazyExprVariant::PrimitiveBinaryOpr {
                opr,
                ref lopd,
                ref ropd,
            } => Ok(opr
                .act_on_primitives(
                    self.husky_feature_eval_expr(lopd)?.primitive(),
                    self.husky_feature_eval_expr(ropd)?.primitive(),
                )?
                .into()),
            FeatureLazyExprVariant::StructOriginalFieldAccess {
                ref this,
                field_idx,
                field_binding,
                opt_linkage,
                field_ident,
                ..
            } => {
                if let Some(linkage) = opt_linkage {
                    let this_value = self.husky_feature_eval_repr(this)?;
                    let this_value = this_value.into_stack()?;
                    let mut arguments = vec![this_value];
                    match catch_unwind(move || linkage.call.0(&mut arguments)) {
                        Ok(result) => todo!(),
                        Err(_) => todo!(),
                    }
                } else {
                    let this_value = self.husky_feature_eval_repr(this)?;
                    match catch_unwind(move || unsafe {
                        this_value.field_access(field_idx, field_binding)
                    }) {
                        Ok(value) => Ok(value),
                        Err(error) => {
                            p!(
                                field_idx,
                                field_ident,
                                this.ty(),
                                expr.expr.ty(),
                                expr.expr.file,
                                expr.expr.range
                            );
                            p!(error);
                            todo!()
                        }
                    }
                }
            }
            FeatureLazyExprVariant::RoutineCall {
                ref opds,
                ref opt_instruction_sheet,
                opt_linkage,
                has_this,
                ..
            } => {
                let result = self.eval_routine_call(
                    opt_instruction_sheet.as_ref().map(|r| &**r),
                    opt_linkage,
                    expr.expr.ty(),
                    opds,
                    has_this,
                );
                result
            }
            FeatureLazyExprVariant::EntityFeature { ref repr, .. } => {
                self.husky_feature_eval_repr(repr)
            }
            FeatureLazyExprVariant::NewRecord {
                ty,
                ref entity,
                ref opds,
            } => {
                todo!()
                // Ok(self
                // .sheet
                // .resolve_class_call(self.db, expr.eval_id, entity, opds)
                // .into()),
            }
            FeatureLazyExprVariant::Variable { ref value, .. } => self
                .cache(EvalKey::Feature(expr.feature), |evaluator: &mut Self| {
                    evaluator.husky_feature_eval_expr(&value)
                }),
            FeatureLazyExprVariant::RecordOriginalFieldAccess {
                ref this,
                field_ident,
                ref repr,
            } => self.husky_feature_eval_repr(repr),
            FeatureLazyExprVariant::ThisValue { ref repr } => self.husky_feature_eval_repr(repr),
            FeatureLazyExprVariant::EvalInput => Ok(self.eval_input.clone()),
            FeatureLazyExprVariant::RecordDerivedFieldAccess {
                ref this,
                field_ident,
                ref repr,
                ..
            } => self.husky_feature_eval_repr(repr),
            FeatureLazyExprVariant::ElementAccess {
                ref opds,
                __Linkage,
                ..
            } => {
                if opds.len() > 2 {
                    todo!()
                }
                let mut values = vec![
                    self.husky_feature_eval_expr(&opds[0])?
                        .into_stack()
                        .unwrap(),
                    self.husky_feature_eval_expr(&opds[1])?
                        .into_stack()
                        .unwrap(),
                ];
                Ok((__Linkage.call.0)(&mut values).into_eval())
            }
            FeatureLazyExprVariant::StructDerivedLazyFieldAccess {
                ref this,
                field_ident,
                ref repr,
            } => {
                let parent = self.husky_feature_eval_repr_cached(this)?.eval_ref();
                let eval_key = EvalKey::StructDerivedField::<'eval> {
                    parent,
                    field_ident: field_ident.ident,
                };
                self.cache(eval_key, |this| this.husky_feature_eval_repr(repr))
            }
            FeatureLazyExprVariant::ModelCall {
                ref opds,
                has_this,
                ref model_defn,
                ref internal,
                ..
            } => match model_defn.variant {
                EntityDefnVariant::Function {
                    ref spatial_parameters,
                    ref parameters,
                    output,
                    ref source,
                } => match source {
                    CallFormSource::Lazy { stmts } => todo!(),
                    CallFormSource::Static(__Linkage::Model(ModelLinkage { eval, .. })) => {
                        let values: Vec<_> = opds
                            .iter()
                            .map(|opd| self.husky_feature_eval_expr(opd))
                            .collect::<__EvalResult<Vec<_>>>()?;
                        eval(internal.as_ref().map_err(|e| e.clone())?, values)
                    }
                    _ => panic!(),
                },
                _ => panic!(),
            },
        }
    }

    pub(crate) fn husky_feature_eval_xml_expr(
        &mut self,
        expr: &FeatureXmlExpr,
    ) -> EvalValueResult<'eval> {
        match expr.variant {
            FeatureXmlExprVariant::Value(ref value_expr) => {
                let this: FeatureRepr = value_expr.clone().into();
                let visual_data = self.visualize_feature(this);
                Ok(EvalValue::Owned(__OwnedValue::new(visual_data?)))
            }
            FeatureXmlExprVariant::Tag {
                tag_kind,
                ref props,
            } => {
                let xml_value = XmlValue {
                    tag_kind,
                    props: props
                        .iter()
                        .map(
                            |(ident, argument)| {
                                self.husky_feature_eval_expr(argument)
                                    .map(|v| (*ident, v.any_ref().to_json_value_dyn()))
                            },
                            // argument.any_ref().to_json_value_dyn()
                        )
                        .collect::<__EvalResult<IdentPairDict<_>>>()?,
                };
                Ok(EvalValue::Owned(__OwnedValue::new(VisualData::from(
                    xml_value.into(),
                ))))
            }
        }
    }

    fn eval_routine_call(
        &mut self,
        opt_instrns: Option<&InstructionSheet>,
        opt_linkage: Option<__Linkage>,
        output_ty: EntityRoutePtr,
        arguments: &[Arc<FeatureExpr>],
        has_this: bool,
    ) -> EvalValueResult<'eval> {
        let db = self.db;
        let verbose = self.vm_config;
        let values = arguments
            .iter()
            .map(|expr| __TempValue::from_eval(self.husky_feature_eval_expr(expr)?));
        msg_once!("kwargs");
        eval_fast(
            db.upcast(),
            opt_instrns,
            opt_linkage,
            output_ty,
            values,
            [].into_iter(),
            verbose,
        )
    }
}
