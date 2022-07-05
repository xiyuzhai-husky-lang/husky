use super::*;
use crate::*;
use husky_entity_route::RangedEntityRoute;
use husky_feature_eval::EvalFeature;
use husky_lazy_semantics::{LazyExprVariant, LazyOpnKind};
use husky_text::RangedCustomIdentifier;
use print_utils::epin;
use vm::InterpreterQueryGroup;
use word::CustomIdentifier;

impl<'a> TraceTokenBuilder<'a> {
    pub(crate) fn gen_feature_expr_tokens(
        &mut self,
        expr: &Arc<FeatureExpr>,
        config: ExprTokenConfig,
    ) {
        let opt_associated_trace_id = if config.associated {
            Some(self.new_trace(None, 0, TraceVariant::FeatureLazyExpr(expr.clone())))
        } else {
            None
        };
        match expr.variant {
            FeatureLazyExprVariant::PrimitiveLiteral(value) => self.push(literal!(value)),
            FeatureLazyExprVariant::PrimitiveBinaryOpr {
                opr,
                ref lopd,
                ref ropd,
            } => {
                self.gen_feature_expr_tokens(lopd, config.subexpr());
                self.push(special!(opr.spaced_code(), opt_associated_trace_id));
                self.gen_feature_expr_tokens(ropd, config.subexpr())
            }
            FeatureLazyExprVariant::Variable { varname, .. } => {
                self.push(ident!(varname.0, opt_associated_trace_id))
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
                            opt_associated_trace_id,
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
                        opt_associated_trace_id,
                        config,
                    ),
                    LazyOpnKind::StructCall(_) => todo!(),
                    LazyOpnKind::RecordCall(_) => todo!(),
                    LazyOpnKind::FieldAccess {
                        field_ident,
                        field_binding,
                    } => todo!(),
                    LazyOpnKind::MethodCall {
                        method_ident,
                        method_route,
                        output_binding,
                    } => todo!(),
                    LazyOpnKind::ElementAccess { element_binding } => todo!(),
                    _ => panic!(),
                },
                _ => panic!(),
            },
            FeatureLazyExprVariant::EnumKindLiteral { .. } => todo!(),
            FeatureLazyExprVariant::EntityFeature { .. } => {
                let text = self
                    .eval_time_singleton
                    .compile_time()
                    .text(expr.expr.file)
                    .unwrap();
                self.push(route!(
                    text.ranged(expr.expr.range),
                    opt_associated_trace_id
                ))
            }
            FeatureLazyExprVariant::NewRecord { ty, ref opds, .. } => todo!(),
            FeatureLazyExprVariant::ThisValue { ref repr } => todo!(),
            FeatureLazyExprVariant::EvalInput => self.push(keyword!("input")),
            FeatureLazyExprVariant::ElementAccess { ref opds, .. } => {
                self.gen_feature_expr_tokens(&opds[0], config.subexpr());
                self.push(special!("[", opt_associated_trace_id.clone()));
                for i in 1..opds.len() {
                    let index_opd = &opds[i];
                    self.gen_feature_expr_tokens(index_opd, config.subexpr());
                }
                self.push(special!("]", opt_associated_trace_id))
            }
            FeatureLazyExprVariant::RecordDerivedFieldAccess {
                ref this,
                field_ident,
                ..
            } => self.gen_feature_eager_field_access_tokens(config, this, field_ident),
            FeatureLazyExprVariant::StructOriginalFieldAccess {
                ref this,
                field_ident,
                ..
            } => self.gen_feature_eager_field_access_tokens(config, this, field_ident),
            FeatureLazyExprVariant::RecordOriginalFieldAccess {
                ref this,
                field_ident,
                ..
            } => self.gen_feature_eager_field_access_tokens(config, this, field_ident),
            FeatureLazyExprVariant::StructDerivedLazyFieldAccess {
                ref this,
                field_ident,
                ..
            } => self.gen_feature_lazy_field_access_tokens(
                config,
                this,
                field_ident,
                opt_associated_trace_id,
            ),
        }
    }

    fn gen_feature_eager_field_access_tokens(
        &mut self,
        config: ExprTokenConfig,
        this: &FeatureRepr,
        field_ident: RangedCustomIdentifier,
    ) {
        match this {
            FeatureRepr::Expr(this) => {
                self.gen_feature_expr_tokens(this, config);
                self.extend([special!("."), ident!(field_ident.ident.as_str())])
            }
            _ => self.push(ident!(field_ident.ident.as_str())),
        }
    }

    fn gen_feature_lazy_field_access_tokens(
        &mut self,
        config: ExprTokenConfig,
        this: &FeatureRepr,
        field_ident: RangedCustomIdentifier,
        opt_associated_trace_id: Option<TraceId>,
    ) {
        match this {
            FeatureRepr::Expr(this) => {
                self.gen_feature_expr_tokens(this, config);
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
        inputs: &[Arc<FeatureExpr>],
        opt_associated_trace_id: Option<TraceId>,
        config: ExprTokenConfig,
    ) {
        let text = self.eval_time().compile_time().text(file).unwrap();
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
