use crate::*;
use feature_gen::*;
use husky_tracer_protocol::VisualData;
use print_utils::{epin, msg_once, p};
use semantics_entity::{CallFormSource, EntityDefnVariant};
use semantics_lazy::LazyStmt;
use static_defn::LinkageSource;
use std::{iter::zip, sync::Arc};
use vm::*;
use word::IdentPairDict;

use super::FeatureEvaluator;

impl<'temp, 'eval: 'temp> FeatureEvaluator<'temp, 'eval> {
    pub(crate) fn eval_feature_lazy_expr(
        &mut self,
        expr: &FeatureLazyExpr,
    ) -> EvalValueResult<'eval> {
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
                    self.eval_feature_lazy_expr(lopd)?.primitive(),
                    self.eval_feature_lazy_expr(ropd)?.primitive(),
                )?
                .into()),
            FeatureLazyExprVariant::StructOriginalFieldAccess {
                ref this,
                field_idx,
                field_binding,
                opt_linkage: opt_compiled,
                ..
            } => {
                if let Some(compiled) = opt_compiled {
                    todo!()
                } else {
                    let this_value = self.eval_feature_repr(this)?;
                    Ok(unsafe { this_value.field_access(field_idx, field_binding) })
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
                    opds,
                    has_this,
                );
                result
            }
            FeatureLazyExprVariant::EntityFeature { ref repr, .. } => self.eval_feature_repr(repr),
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
                    evaluator.eval_feature_lazy_expr(&value)
                }),
            FeatureLazyExprVariant::RecordOriginalFieldAccess {
                ref this,
                field_ident,
                ref repr,
            } => self.eval_feature_repr(repr),
            FeatureLazyExprVariant::ThisValue { ref repr } => self.eval_feature_repr(repr),
            FeatureLazyExprVariant::EvalInput => Ok(self.eval_input.clone()),
            FeatureLazyExprVariant::RecordDerivedFieldAccess {
                ref this,
                field_ident,
                ref repr,
                ..
            } => self.eval_feature_repr(repr),
            FeatureLazyExprVariant::ElementAccess {
                ref opds, linkage, ..
            } => {
                if opds.len() > 2 {
                    todo!()
                }
                let mut values = vec![
                    self.eval_feature_lazy_expr(&opds[0])?.into_stack().unwrap(),
                    self.eval_feature_lazy_expr(&opds[1])?.into_stack().unwrap(),
                ];
                (linkage.call)(&mut values).map(|mut value| value.into_eval())
            }
            FeatureLazyExprVariant::StructDerivedLazyFieldAccess {
                ref this,
                field_ident,
                ref repr,
            } => {
                let parent = self.eval_feature_repr_cached(this)?.eval_ref();
                let eval_key = EvalKey::StructDerivedField::<'eval> {
                    parent,
                    field_ident: field_ident.ident,
                };
                self.cache(eval_key, |this| this.eval_feature_repr(repr))
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
                    CallFormSource::Static(LinkageSource::Model(ModelLinkage { eval, .. })) => {
                        let values: Vec<_> = opds
                            .iter()
                            .map(|opd| self.eval_feature_lazy_expr(opd))
                            .collect::<EvalResult<Vec<_>>>()
                            .unwrap();
                        eval(internal.as_ref().map_err(|e| e.clone())?, values)
                    }
                    _ => panic!(),
                },
                _ => panic!(),
            },
        }
    }

    pub(crate) fn eval_feature_xml_expr(
        &mut self,
        expr: &FeatureXmlExpr,
    ) -> EvalValueResult<'eval> {
        match expr.variant {
            FeatureXmlExprVariant::Value(ref value_expr) => {
                let this: FeatureRepr = value_expr.clone().into();
                let visual_data = self.visualize(this);
                Ok(EvalValue::Owned(OwnedValue::new(visual_data?)))
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
                                self.eval_feature_lazy_expr(argument)
                                    .map(|v| (*ident, v.any_ref().to_json_value_dyn()))
                            },
                            // argument.any_ref().to_json_value_dyn()
                        )
                        .collect::<EvalResult<IdentPairDict<_>>>()?,
                };
                Ok(EvalValue::Owned(OwnedValue::new(VisualData::from(
                    xml_value.into(),
                ))))
            }
        }
    }

    fn eval_routine_call(
        &mut self,
        opt_instrns: Option<&InstructionSheet>,
        opt_linkage: Option<RoutineLinkage>,
        arguments: &[Arc<FeatureLazyExpr>],
        has_this: bool,
    ) -> EvalValueResult<'eval> {
        let db = self.db;
        let verbose = self.verbose;
        let values = arguments
            .iter()
            .map(|expr| TempValue::from_eval(self.eval_feature_lazy_expr(expr)?));
        msg_once!("kwargs");
        eval_fast(
            db.upcast(),
            opt_instrns,
            opt_linkage,
            values,
            [].into_iter(),
            verbose,
        )
    }
}
