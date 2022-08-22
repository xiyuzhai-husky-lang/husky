use crate::*;
use husky_entity_route::EntityRoutePtr;
use husky_entity_semantics::{CallFormSource, EntityDefnVariant};
use husky_feature_gen::*;
use husky_lazy_semantics::LazyStmt;
use husky_pattern_semantics::{PurePattern, PurePatternVariant};
use husky_print_utils::{epin, msg_once, p};
use husky_trace_protocol::VisualData;
use husky_word::IdentPairDict;
use husky_xml_syntax::XmlValue;
use std::{iter::zip, panic::catch_unwind, sync::Arc};
use vm::__Linkage;
use vm::*;

use super::FeatureEvaluator;

impl<'temp, 'eval: 'temp> FeatureEvaluator<'temp, 'eval> {
    pub(crate) fn eval_expr(&self, expr: &FeatureLazyExpr) -> __VMResult<__Register<'eval>> {
        match expr.variant {
            FeatureLazyExprVariant::Literal(ref value) => Ok(value.clone()),
            FeatureLazyExprVariant::PrimitiveBinaryOpr {
                linkage, ref opds, ..
            } => self.eval_routine_call(&None, Some(linkage), expr.expr.ty(), opds),
            FeatureLazyExprVariant::StructOriginalField {
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
            FeatureLazyExprVariant::RoutineCall {
                ref opds,
                ref opt_instruction_sheet,
                opt_linkage,
                has_this,
                ..
            } => self.eval_routine_call(opt_instruction_sheet, opt_linkage, expr.expr.ty(), opds),
            FeatureLazyExprVariant::EntityFeature { ref repr } => {
                self.eval_feature_repr_cached(repr)
            }
            FeatureLazyExprVariant::NewRecord {
                ty,
                ref entity,
                ref opds,
            } => todo!(),
            // Ok(self
            //     .sheet
            //     .resolve_record_call(self.db, expr.eval_id, entity, opds)
            //     .into()),
            FeatureLazyExprVariant::Variable { ref value, .. } => self
                .cache(EvalKey::Feature(expr.feature), |evaluator: &Self| {
                    evaluator.eval_expr(&value)
                }),
            FeatureLazyExprVariant::RecordOriginalField {
                ref this,
                field_ident,
                ref repr,
            } => self.eval_feature_repr(repr),
            FeatureLazyExprVariant::ThisValue { ref repr } => self.eval_feature_repr(repr),
            FeatureLazyExprVariant::EvalInput => Ok(self.target_input.clone()),
            FeatureLazyExprVariant::RecordDerivedField { ref repr, .. } => {
                self.eval_feature_repr(repr)
            }
            FeatureLazyExprVariant::ElementAccess {
                ref opds, linkage, ..
            } => {
                if opds.len() > 2 {
                    todo!()
                }
                let values = vec![self.eval_expr(&opds[0])?, self.eval_expr(&opds[1])?];
                linkage.call_catch_unwind(unsafe { self.some_ctx() }, values)
            }
            FeatureLazyExprVariant::StructDerivedLazyField {
                ref this,
                field_ident,
                field_uid,
                ref repr,
            } => {
                let parent = unsafe { self.eval_feature_repr_cached(this)?.data().as_ptr };
                let eval_key = EvalKey::StructDerivedField {
                    this: parent,
                    field_uid,
                };
                let result = self.cache(eval_key, |this| this.eval_feature_repr(repr));
                result
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
                    CallFormSource::Static(__Linkage::Model(model)) => {
                        let values: Vec<_> = opds
                            .iter()
                            .map(|opd| self.eval_expr(opd))
                            .collect::<__VMResult<Vec<_>>>()?;
                        model.eval_dyn(internal.as_ref().map_err(|e| e.clone())?, &values)
                    }
                    _ => panic!(),
                },
                _ => panic!(),
            },
            FeatureLazyExprVariant::NewVecFromList {
                ref elements,
                linkage,
            } => self.eval_routine_call(&None, Some(linkage), expr.expr.ty(), elements),
            FeatureLazyExprVariant::CustomBinaryOpr {
                opr,
                ref opds,
                ref opt_instruction_sheet,
                opt_linkage,
            } => self.eval_routine_call(opt_instruction_sheet, opt_linkage, expr.expr.ty(), opds),
            FeatureLazyExprVariant::BePattern { ref this, ref patt } => {
                self.eval_be_pattern(this, patt)
            }
        }
    }

    pub(crate) fn eval_expr_cached(&self, expr: &FeatureLazyExpr) -> __VMResult<__Register<'eval>> {
        let eval_key = EvalKey::Feature(expr.feature);
        if let Some(result) = self.sheet.cached_value(eval_key) {
            result
        } else {
            let result = self.eval_expr(expr);
            match expr.variant {
                FeatureLazyExprVariant::EntityFeature {
                    repr: FeatureRepr::TargetInput { .. },
                } => result, // ad hoc
                _ => self.sheet.try_cache(eval_key, result),
            }
        }
    }

    fn eval_struct_original_field(
        &self,
        opt_linkage: Option<__ResolvedLinkage>,
        this: &FeatureRepr,
        field_idx: u8,
        field_binding: Binding,
        field_ident: husky_text::RangedCustomIdentifier,
        expr: &FeatureLazyExpr,
    ) -> __VMResult<__Register<'eval>> {
        if let Some(linkage) = opt_linkage {
            let this_value = self.eval_feature_repr(this)?;
            linkage.call_catch_unwind(unsafe { self.some_ctx() }, vec![this_value])
        } else {
            let this_value = self.eval_feature_repr(this)?;
            match catch_unwind(move || unsafe {
                assert_eq!(
                    this_value.vtable as *const _,
                    &__VIRTUAL_STRUCT_VTABLE as *const _
                );
                match field_binding {
                    Binding::EvalRef => todo!(),
                    Binding::TempRef => todo!(),
                    Binding::TempMut => todo!(),
                    Binding::Move => todo!(),
                    Binding::Copy => {
                        let this_value: &VirtualStruct =
                            this_value.downcast_temp_ref(&__VIRTUAL_STRUCT_VTABLE);
                        this_value.bind_field_copy(field_idx)
                    }
                    Binding::DerefCopy => todo!(),
                }
                // this_value.field_access(field_idx, field_binding)
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

    pub(crate) fn eval_xml_expr(&self, expr: &FeatureXmlExpr) -> __VMResult<__Register<'eval>> {
        match expr.variant {
            FeatureXmlExprVariant::Value(ref value_expr) => {
                let this: FeatureRepr = value_expr.clone().into();
                let visual_data = self.visualize_feature(this)?;
                Ok(__Register::new_box(visual_data, &__VISUAL_DATA_VTABLE))
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
                                self.eval_expr(argument).map(|v| {
                                    (
                                        *ident,
                                        self.serialize(self.db.comptime(), &v, argument.expr.ty()),
                                    )
                                })
                            },
                            // argument.any_ref().to_json_value_dyn()
                        )
                        .collect::<__VMResult<IdentPairDict<_>>>()?,
                };

                Ok(__Register::new_box(
                    VisualData::from(xml_value.into()),
                    &__VISUAL_DATA_VTABLE,
                ))
            }
        }
    }

    fn eval_routine_call(
        &self,
        opt_instrns: &Option<Arc<InstructionSheet>>,
        opt_linkage: Option<__Linkage>,
        output_ty: EntityRoutePtr,
        arguments: &[Arc<FeatureLazyExpr>],
    ) -> __VMResult<__Register<'eval>> {
        let db = self.db;
        let vm_config = self.vm_config();
        let values = arguments
            .iter()
            .map(|expr| self.eval_expr(expr))
            .collect::<Vec<_>>();
        msg_once!("kwargs");
        eval_fast(
            db.upcast(),
            Some(self),
            opt_instrns.as_ref().map(|v| &**v),
            opt_linkage,
            output_ty,
            values.into_iter(),
            [].into_iter(),
            arguments.len().try_into().unwrap(),
            vm_config,
        )
    }

    fn eval_be_pattern(
        &self,
        this: &FeatureLazyExpr,
        patt: &PurePattern,
    ) -> __VMResult<__Register<'eval>> {
        let this_value = self.eval_expr(this)?;
        Ok(match patt.variant {
            PurePatternVariant::PrimitiveLiteral(_) => todo!(),
            PurePatternVariant::OneOf { ref subpatterns } => todo!(),
            PurePatternVariant::EnumLiteral(_) => todo!(),
            PurePatternVariant::Some => match this_value.data_kind() {
                __RegisterDataKind::PrimitiveValue => true.to_register(),
                __RegisterDataKind::Box => todo!(),
                __RegisterDataKind::EvalRef => todo!(),
                __RegisterDataKind::TempRef => todo!(),
                __RegisterDataKind::TempMut => todo!(),
                __RegisterDataKind::Moved => todo!(),
                __RegisterDataKind::None => todo!(),
                __RegisterDataKind::Unreturned => todo!(),
            },
            PurePatternVariant::None => todo!(),
        })
    }
}
