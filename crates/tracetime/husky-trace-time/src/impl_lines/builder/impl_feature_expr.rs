use super::*;
use crate::*;
use husky_entity_route::RangedEntityRoute;
use husky_feature_eval::EvalFeature;
use husky_lazy_semantics::{LazyExprVariant, LazyOpnKind};
use husky_print_utils::epin;
use husky_text::RangedCustomIdentifier;
use husky_vm::InterpreterQueryGroup;
use husky_word::CustomIdentifier;

impl<'a> TraceTokenBuilder<'a> {
    pub(crate) fn gen_feature_expr_tokens(
        &mut self,
        expr: &Arc<FeatureLazyExpr>,
        config: ExprTokenConfig,
    ) {
        let opt_assoc_id = if config.associated {
            Some(self.new_trace(None, 0, TraceVariant::FeatureExpr(expr.clone())))
        } else {
            None
        };
        match expr.variant {
            FeatureLazyExprVariant::Literal(_) => match expr.expr.variant {
                LazyExprVariant::PrimitiveLiteral(value) => self.push(literal!(value)),
                LazyExprVariant::EnumLiteral { entity_route } => {
                    let text = self.runtime().comptime().text(expr.expr.file).unwrap();
                    self.push(route!(text.ranged(expr.expr.range), opt_assoc_id))
                }
                _ => panic!(),
            },
            FeatureLazyExprVariant::PrimitiveBinaryOpr { opr, ref opds, .. } => {
                self.gen_feature_expr_tokens(&opds[0], config.subexpr());
                self.push(special!(opr.spaced_code(), opt_assoc_id));
                self.gen_feature_expr_tokens(&opds[1], config.subexpr())
            }
            FeatureLazyExprVariant::CustomBinaryOpr { opr, ref opds, .. } => {
                self.gen_feature_expr_tokens(&opds[0], config.subexpr());
                self.push(special!(opr.spaced_code(), opt_assoc_id));
                self.gen_feature_expr_tokens(&opds[1], config.subexpr())
            }
            FeatureLazyExprVariant::Variable { varname, .. } => {
                self.push(ident!(varname.0, opt_assoc_id))
            }
            FeatureLazyExprVariant::RoutineCall {
                opds: ref feature_opds,
                ..
            } => match expr.expr.variant {
                LazyExprVariant::Opn { opn_kind, ref opds } => match opn_kind {
                    LazyOpnKind::FunctionRoutineCall(ranged_route) => self
                        .feature_entity_call_tokens(
                            expr.expr.file,
                            ranged_route,
                            feature_opds,
                            opt_assoc_id,
                            config,
                        ),
                    LazyOpnKind::StructCall(_) => todo!(),
                    LazyOpnKind::RecordCall(_) => todo!(),
                    LazyOpnKind::MethodCall {
                        method_ident,
                        method_route,
                        output_binding,
                    } => {
                        self.gen_feature_expr_tokens(&feature_opds[0], config.subexpr());
                        self.push(special!("."));
                        self.push(ident!(method_ident.ident.0));
                        self.push(special!("("));
                        for i in 1..opds.len() {
                            if i > 1 {
                                self.push(special!(", "))
                            }
                            self.gen_feature_expr_tokens(&feature_opds[i], config.subexpr());
                        }
                        self.push(special!(")"));
                    }
                    _ => panic!(),
                },
                _ => panic!(""),
            },
            FeatureLazyExprVariant::ModelCall {
                ref opds,
                has_this,
                ref model_defn,
                ..
            } => match expr.expr.variant {
                LazyExprVariant::Opn { opn_kind, .. } => match opn_kind {
                    LazyOpnKind::FunctionModelCall(route) => self.feature_entity_call_tokens(
                        expr.expr.file,
                        route,
                        opds,
                        opt_assoc_id,
                        config,
                    ),
                    LazyOpnKind::StructCall(_) => todo!(),
                    LazyOpnKind::RecordCall(_) => todo!(),
                    LazyOpnKind::Field {
                        field_ident,
                        field_binding,
                    } => todo!(),
                    LazyOpnKind::MethodCall {
                        method_ident,
                        method_route,
                        output_binding,
                    } => todo!(),
                    LazyOpnKind::Index { element_binding } => todo!(),
                    _ => panic!(),
                },
                _ => panic!(),
            },
            FeatureLazyExprVariant::EntityFeature { .. } => {
                let text = self.runtime.comptime().text(expr.expr.file).unwrap();
                self.push(route!(text.ranged(expr.expr.range), opt_assoc_id))
            }
            FeatureLazyExprVariant::NewRecord { ty, ref opds, .. } => todo!(),
            FeatureLazyExprVariant::ThisValue { ref repr } => todo!(),
            FeatureLazyExprVariant::EvalInput => self.push(keyword!("input")),
            FeatureLazyExprVariant::ElementAccess { ref opds, .. } => {
                self.gen_feature_expr_tokens(&opds[0], config.subexpr());
                self.push(special!("[", opt_assoc_id.clone()));
                for i in 1..opds.len() {
                    let index_opd = &opds[i];
                    self.gen_feature_expr_tokens(index_opd, config.subexpr());
                }
                self.push(special!("]", opt_assoc_id))
            }
            FeatureLazyExprVariant::RecordDerivedField {
                ref this,
                field_ident,
                ..
            } => self.gen_feature_eager_field_tokens(config, this, field_ident),
            FeatureLazyExprVariant::StructOriginalField {
                ref this,
                field_ident,
                ..
            } => self.gen_feature_eager_field_tokens(config, this, field_ident),
            FeatureLazyExprVariant::RecordOriginalField {
                ref this,
                field_ident,
                ..
            } => self.gen_feature_eager_field_tokens(config, this, field_ident),
            FeatureLazyExprVariant::StructDerivedLazyField {
                ref this,
                field_ident,
                ..
            } => self.gen_feature_lazy_field_tokens(this, field_ident, opt_assoc_id, config),
            FeatureLazyExprVariant::NewVecFromList { ref elements, .. } => {
                self.gen_new_vec_from_list_tokens(elements, opt_assoc_id, config)
            }
            FeatureLazyExprVariant::BePattern {
                ref this,
                patt: ref pure_pattern,
            } => todo!(),
        }
    }

    fn gen_new_vec_from_list_tokens(
        &mut self,
        elements: &[Arc<FeatureLazyExpr>],
        opt_assoc_id: Option<TraceId>,
        config: ExprTokenConfig,
    ) {
        self.push(special!("[", opt_assoc_id));
        for (i, element) in elements.iter().enumerate() {
            if i > 0 {
                self.push(special!(", "))
            }
            self.gen_feature_expr_tokens(element, config.subexpr())
        }
        self.push(special!("]", opt_assoc_id));
    }

    fn gen_feature_eager_field_tokens(
        &mut self,
        config: ExprTokenConfig,
        this: &FeatureRepr,
        field_ident: RangedCustomIdentifier,
    ) {
        match this {
            FeatureRepr::LazyExpr(this) => {
                self.gen_feature_expr_tokens(this, config.subexpr());
                self.extend([special!("."), ident!(field_ident.ident.as_str())])
            }
            _ => self.push(ident!(field_ident.ident.as_str())),
        }
    }

    fn gen_feature_lazy_field_tokens(
        &mut self,
        this: &FeatureRepr,
        field_ident: RangedCustomIdentifier,
        opt_associated_trace_id: Option<TraceId>,
        config: ExprTokenConfig,
    ) {
        match this {
            FeatureRepr::LazyExpr(this) => {
                self.gen_feature_expr_tokens(this, config.subexpr());
                self.extend([
                    special!("."),
                    ident!(field_ident.ident.as_str(), opt_associated_trace_id),
                ])
            }
            _ => self.push(ident!(field_ident.ident.as_str())),
        }
    }

    fn feature_entity_call_tokens(
        &mut self,
        file: FilePtr,
        ranged_scope: RangedEntityRoute,
        inputs: &[Arc<FeatureLazyExpr>],
        opt_associated_trace_id: Option<TraceId>,
        config: ExprTokenConfig,
    ) {
        let text = self.runtime().comptime().text(file).unwrap();
        self.extend([
            route!(text.ranged(ranged_scope.range), opt_associated_trace_id),
            special!("("),
        ]);
        for (i, input) in inputs.iter().enumerate() {
            if i > 0 {
                self.push(special!(", "));
            }
            self.gen_feature_expr_tokens(input, config.subexpr());
        }
        self.push(special!(")"))
    }
}
