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
    pub(crate) fn eval_expr(&mut self, expr: &FeatureExpr) -> __EvalValueResult<'eval> {
        match expr.variant {
            FeatureExprVariant::PrimitiveLiteral(value) => Ok(value.into()),
            FeatureExprVariant::EnumKindLiteral { entity_route, uid } => {
                todo!()
            }
            FeatureExprVariant::PrimitiveBinaryOpr {
                opr,
                ref lopd,
                ref ropd,
            } => Ok(opr
                .act_on_primitives(
                    self.eval_expr(lopd)?.primitive(),
                    self.eval_expr(ropd)?.primitive(),
                )?
                .into()),
            FeatureExprVariant::StructOriginalField {
                ref this,
                field_idx,
                field_binding,
                opt_linkage,
                field_ident,
                ..
            } => self.eval_struct_original_field(
                opt_linkage,
                this,
                field_idx,
                field_binding,
                field_ident,
                expr,
            ),
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
                    expr.expr.ty(),
                    opds,
                    has_this,
                );
                result
            }
            FeatureExprVariant::EntityFeature { ref repr } => self.eval_feature_repr_cached(repr),
            FeatureExprVariant::NewRecord {
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
            FeatureExprVariant::Variable { ref value, .. } => self
                .cache(EvalKey::Feature(expr.feature), |evaluator: &mut Self| {
                    evaluator.eval_expr(&value)
                }),
            FeatureExprVariant::RecordOriginalField {
                ref this,
                field_ident,
                ref repr,
            } => self.eval_feature_repr(repr),
            FeatureExprVariant::ThisValue { ref repr } => self.eval_feature_repr(repr),
            FeatureExprVariant::EvalInput => Ok(self.eval_input.clone()),
            FeatureExprVariant::RecordDerivedField { ref repr, .. } => self.eval_feature_repr(repr),
            FeatureExprVariant::ElementAccess {
                ref opds, linkage, ..
            } => {
                if opds.len() > 2 {
                    todo!()
                }
                let mut values = vec![
                    self.eval_expr(&opds[0])?.into_stack().unwrap(),
                    self.eval_expr(&opds[1])?.into_stack().unwrap(),
                ];
                linkage.eval(unsafe { self.some_ctx() }, values)
            }
            FeatureExprVariant::StructDerivedLazyField {
                ref this,
                field_ident,
                field_uid,
                ref repr,
            } => {
                let parent = self.eval_feature_repr_cached(this)?.eval_ref();
                let eval_key = EvalKey::StructDerivedField::<'eval> { parent, field_uid };
                self.cache(eval_key, |this| this.eval_feature_repr(repr))
            }
            FeatureExprVariant::ModelCall {
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
                    CallFormSource::Static(__Linkage::Model(model)) => {
                        let values: Vec<_> = opds
                            .iter()
                            .map(|opd| self.eval_expr(opd))
                            .collect::<__EvalResult<Vec<_>>>()?;
                        model.eval_dyn(internal.as_ref().map_err(|e| e.clone())?, &values)
                    }
                    _ => panic!(),
                },
                _ => panic!(),
            },
            FeatureExprVariant::NewVecFromList { .. } => todo!(),
        }
    }

    fn eval_struct_original_field(
        &mut self,
        opt_linkage: Option<__SpecificRoutineLinkage>,
        this: &FeatureRepr,
        field_idx: usize,
        field_binding: Binding,
        field_ident: husky_text::RangedCustomIdentifier,
        expr: &FeatureExpr,
    ) -> __EvalValueResult<'eval> {
        if let Some(linkage) = opt_linkage {
            let this_value = self.eval_feature_repr(this)?;
            let this_value = this_value.into_stack()?;
            linkage.eval(unsafe { self.some_ctx() }, vec![this_value])
        } else {
            let this_value = self.eval_feature_repr(this)?;
            match catch_unwind(move || unsafe { this_value.field_access(field_idx, field_binding) })
            {
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

    pub(crate) fn eval_xml_expr(&mut self, expr: &FeatureXmlExpr) -> __EvalValueResult<'eval> {
        match expr.variant {
            FeatureXmlExprVariant::Value(ref value_expr) => {
                let this: FeatureRepr = value_expr.clone().into();
                let visual_data = self.visualize_feature(this);
                Ok(__EvalValue::Owned(__OwnedValue::new(visual_data?)))
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
                                self.eval_expr(argument)
                                    .map(|v| (*ident, v.any_ref().__to_json_value_dyn()))
                            },
                            // argument.any_ref().to_json_value_dyn()
                        )
                        .collect::<__EvalResult<IdentPairDict<_>>>()?,
                };
                Ok(__EvalValue::Owned(__OwnedValue::new(VisualData::from(
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
    ) -> __EvalValueResult<'eval> {
        let db = self.db;
        let opt_ctx = unsafe { self.some_ctx() };
        let vm_config = self.vm_config();
        let values = arguments
            .iter()
            .map(|expr| __TempValue::from_eval(self.eval_expr(expr)?));
        msg_once!("kwargs");
        eval_fast(
            db.upcast(),
            opt_ctx,
            opt_instrns,
            opt_linkage,
            output_ty,
            values,
            [].into_iter(),
            vm_config,
        )
    }
}
