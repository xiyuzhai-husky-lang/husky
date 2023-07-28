use crate::*;
use husky_coword::IdentPairMap;
use husky_item_semantics::{CallFormSource, EntityDefnVariant};
use husky_opn_semantics::ImplicitConversion;
use husky_opr::BinaryPureClosedOpr;
use husky_pattern_semantics::{PurePattern, PurePatternVariant};
use husky_print_utils::{msg_once, p};
use husky_text::HasSourceRange;
use husky_trace_protocol::VisualData;
use husky_val_repr::*;
use husky_vm::__LinkageGroup;
use husky_vm::*;
use husky_xml_syntax::HtmlValue;
use std::{panic::catch_unwind, sync::Arc};

use super::FeatureEvaluator;

impl<'temp> FeatureEvaluator<'temp, 'static> {
    pub(crate) fn eval_expr(&self, expr: &FeatureLazyExpr) -> __VMResult<__RegularValue> {
        let result = match expr.variant {
            FeatureLazyExprVariant::Literal(ref value) => Ok(value.clone()),
            FeatureLazyExprVariant::PrimitiveBinaryOpr {
                linkage, ref opds, ..
            } => self.eval_routine_call(&None, Some(linkage), opds),
            FeatureLazyExprVariant::ShortCircuitBinaryOpr { opr, ref opds } => {
                let lopd = self.eval_expr(&opds[0])?.to_bool();
                todo!()
                // match opr {
                //     BinaryShortcuitLogicOpr::And => {
                //         if !lopd {
                //             return Ok((false).to_register());
                //         }
                //         self.eval_expr(&opds[1])
                //     }
                //     BinaryShortcuitLogicOpr::Or => {
                //         if lopd {
                //             return Ok((true).to_register());
                //         }
                //         self.eval_expr(&opds[1])
                //     }
                //     _ => panic!(),
                // }
            }
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
                ..
            } => self.eval_routine_call(opt_instruction_sheet, opt_linkage, opds),
            FeatureLazyExprVariant::EntityFeature { ref repr } => {
                self.eval_feature_repr_cached(repr)
            }
            FeatureLazyExprVariant::NewRecord { .. } => todo!(),
            FeatureLazyExprVariant::Variable { ref value, .. } => self
                .cache(EvalKey::Feature(expr.feature), |evaluator: &Self| {
                    evaluator.eval_expr(&value)
                }),
            FeatureLazyExprVariant::RecordOriginalField { ref repr, .. } => {
                self.eval_feature_repr(repr)
            }
            FeatureLazyExprVariant::ThisValue { ref repr } => self.eval_feature_repr(repr),
            FeatureLazyExprVariant::EvalInput => Ok(self.target_input.clone()),
            FeatureLazyExprVariant::RecordDerivedField { ref repr, .. } => {
                self.eval_feature_repr(repr)
            }
            FeatureLazyExprVariant::Index {
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
                field_uid,
                ref repr,
                ..
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
                ref model_defn,
                ref internal,
                ..
            } => match model_defn.variant {
                EntityDefnVariant::Function { ref source, .. } => match source {
                    CallFormSource::Lazy { .. } => todo!(),
                    CallFormSource::Static(__LinkageGroup::Model(model)) => {
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
            } => self.eval_routine_call(&None, Some(linkage), elements),
            FeatureLazyExprVariant::CustomBinaryOpr {
                ref opds,
                ref opt_instruction_sheet,
                opt_linkage,
                ..
            } => self.eval_routine_call(opt_instruction_sheet, opt_linkage, opds),
            FeatureLazyExprVariant::BePattern { ref this, ref patt } => {
                self.eval_be_pattern(this, patt)
            }
            FeatureLazyExprVariant::PrefixOpr {
                opr,
                ref opds,
                linkage,
            } => self.eval_routine_call(&None, Some(linkage), opds),
        };
        match expr.expr.implicit_conversion {
            ImplicitConversion::None => result,
            ImplicitConversion::WrapInSome { number_of_somes } => {
                result.map(|v| v.wrap_in_some(number_of_somes))
            }
            ImplicitConversion::ConvertToBool => todo!(),
        }
    }

    pub(crate) fn eval_expr_cached(&self, expr: &FeatureLazyExpr) -> __VMResult<__RegularValue> {
        match expr.variant {
            FeatureLazyExprVariant::EntityFeature {
                repr: ValRepr::TargetInput { .. },
            } => self.eval_expr(expr), // ad hoc
            _ => {
                let eval_key = EvalKey::Feature(expr.feature);
                self.eval_cached(eval_key, |this| this.eval_expr(expr))
            }
        }
    }

    fn eval_struct_original_field(
        &self,
        opt_linkage: Option<__ResolvedLinkage>,
        this: &ValRepr,
        field_idx: u8,
        field_binding: Binding,
        field_ident: husky_text::RangedIdent,
        expr: &FeatureLazyExpr,
    ) -> __VMResult<__RegularValue> {
        if let Some(linkage) = opt_linkage {
            let this_value = self.eval_feature_repr(this)?;
            linkage
                .call_catch_unwind(unsafe { self.some_ctx() }, vec![this_value])
                .map_err(|e| {
                    e.with_message(&format!(
                        "while evaluating struct original field with field binding `{field_binding:?}` at {}", expr.source_range()
                    ))
                })
        } else {
            let this_value = self.eval_feature_repr(this)?;
            match catch_unwind(move || {
                assert_eq!(
                    this_value.vtable as *const _,
                    &__DEPRECATED_VIRTUAL_STRUCT_VTABLE as *const _
                );
                match field_binding {
                    Binding::Leash => todo!(),
                    Binding::TempRef => todo!(),
                    Binding::TempMut => todo!(),
                    Binding::Move => todo!(),
                    Binding::Copy => {
                        let this_value: &DeprecatedVirtualStruct =
                            this_value.downcast_temp_ref(&__DEPRECATED_VIRTUAL_STRUCT_VTABLE);
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
                        field_binding,
                        this.ty(),
                        expr.expr.intrinsic_ty(),
                        expr.expr.file,
                        expr.expr.range
                    );
                    p!(error);
                    todo!()
                }
            }
        }
    }

    pub(crate) fn eval_xml_expr(&self, expr: &FeatureHtmlExpr) -> __VMResult<__RegularValue> {
        match expr.variant {
            FeatureHtmlExprVariant::Value(ref value_expr) => {
                let this: ValRepr = value_expr.clone().into();
                let visual_data = self.visualize_feature(this)?;
                Ok(__RegularValue::new_box(visual_data, &__VISUAL_DATA_VTABLE))
            }
            FeatureHtmlExprVariant::Tag {
                tag_kind,
                ref props,
            } => {
                let xml_value = HtmlValue {
                    tag_kind,
                    props: props
                        .iter()
                        .map(
                            |(ident, argument)| {
                                self.eval_expr(argument).map(|v| {
                                    (*ident, self.serialize(&v, argument.expr.intrinsic_ty()))
                                })
                            },
                            // argument.any_ref().to_json_value_dyn()
                        )
                        .collect::<__VMResult<IdentPairMap<_>>>()?,
                };

                Ok(__RegularValue::new_box(
                    VisualData::from(xml_value.into()),
                    &__VISUAL_DATA_VTABLE,
                ))
            }
        }
    }

    fn eval_routine_call(
        &self,
        opt_instrns: &Option<Instructions>,
        opt_linkage: Option<__LinkageGroup>,
        arguments: &[ValExpr],
    ) -> __VMResult<__RegularValue> {
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
    ) -> __VMResult<__RegularValue> {
        let this_value = self.eval_expr(this)?;
        Ok(match patt.variant {
            PurePatternVariant::PrimitiveLiteral(_) => todo!(),
            PurePatternVariant::OneOf { .. } => todo!(),
            PurePatternVariant::EnumLiteral(_) => todo!(),
            PurePatternVariant::Some => this_value.is_some().to_register(),
            PurePatternVariant::None => this_value.is_none().to_register(),
        })
    }
}
