use super::impl_tokens::ExprTokenConfig;
use super::*;
use crate::*;
use entity_route::RangedEntityRoute;
use eval_feature::EvalFeature;
use print_utils::epin;
use semantics_lazy::{LazyExprVariant, LazyOpnKind};
use text::RangedCustomIdentifier;
use visualizer_gen::VisualTy;
use vm::InterpreterQueryGroup;
use word::CustomIdentifier;

impl HuskyTraceTime {
    pub(crate) fn feature_expr_tokens(
        &mut self,
        expr: &Arc<FeatureLazyExpr>,
        config: ExprTokenConfig,
    ) -> Vec<TraceTokenData> {
        let opt_associated_trace_id = if config.associated {
            Some(self.new_trace(None, 0, TraceVariant::FeatureExpr(expr.clone())))
        } else {
            None
        };
        return match expr.variant {
            FeatureLazyExprVariant::PrimitiveLiteral(value) => vec![literal!(value)],
            FeatureLazyExprVariant::PrimitiveBinaryOpr {
                opr,
                ref lopd,
                ref ropd,
            } => {
                let mut tokens = vec![];
                tokens.extend(self.feature_expr_tokens(lopd, config.subexpr()));
                tokens.push(special!(opr.spaced_code(), opt_associated_trace_id));
                tokens.extend(self.feature_expr_tokens(ropd, config.subexpr()));
                tokens
            }
            FeatureLazyExprVariant::Variable { varname, .. } => {
                vec![ident!(varname.0, opt_associated_trace_id)]
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
                        let mut tokens = vec![];
                        tokens.extend(self.feature_expr_tokens(&feature_opds[0], config.subexpr()));
                        tokens.push(special!("."));
                        tokens.push(ident!(method_ident.ident.0));
                        tokens.push(special!("("));
                        for i in 1..opds.len() {
                            if i > 1 {
                                tokens.push(special!(", "))
                            }
                            tokens.extend(
                                self.feature_expr_tokens(&feature_opds[i], config.subexpr()),
                            );
                        }
                        tokens.push(special!(")"));
                        tokens
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
                vec![route!(
                    text.ranged(expr.expr.range),
                    opt_associated_trace_id
                )]
            }
            FeatureLazyExprVariant::NewRecord { ty, ref opds, .. } => todo!(),
            FeatureLazyExprVariant::ThisValue { ref repr } => todo!(),
            FeatureLazyExprVariant::EvalInput => vec![keyword!("input")],
            FeatureLazyExprVariant::ElementAccess { ref opds, .. } => {
                let mut tokens = vec![];
                tokens.extend(self.feature_expr_tokens(&opds[0], config.subexpr()));
                tokens.push(special!("[", opt_associated_trace_id.clone()));
                for i in 1..opds.len() {
                    let index_opd = &opds[i];
                    tokens.extend(self.feature_expr_tokens(index_opd, config.subexpr()));
                }
                tokens.push(special!("]", opt_associated_trace_id));
                tokens
            }
            FeatureLazyExprVariant::RecordDerivedFieldAccess {
                ref this,
                field_ident,
                ..
            } => self.feature_eager_field_access_tokens(config, this, field_ident),
            FeatureLazyExprVariant::StructOriginalFieldAccess {
                ref this,
                field_ident,
                ..
            } => self.feature_eager_field_access_tokens(config, this, field_ident),
            FeatureLazyExprVariant::RecordOriginalFieldAccess {
                ref this,
                field_ident,
                ..
            } => self.feature_eager_field_access_tokens(config, this, field_ident),
            FeatureLazyExprVariant::StructDerivedLazyFieldAccess {
                ref this,
                field_ident,
                ..
            } => self.feature_lazy_field_access_tokens(
                config,
                this,
                field_ident,
                opt_associated_trace_id,
            ),
        };
    }

    fn feature_eager_field_access_tokens(
        &mut self,
        config: ExprTokenConfig,
        this: &FeatureRepr,
        field_ident: RangedCustomIdentifier,
    ) -> Vec<TraceTokenData> {
        match this {
            FeatureRepr::Expr(this) => {
                let mut tokens = self.feature_expr_tokens(this, config);
                tokens.extend([special!("."), ident!(field_ident.ident.as_str())]);
                tokens
            }
            _ => vec![ident!(field_ident.ident.as_str())],
        }
    }

    fn feature_lazy_field_access_tokens(
        &mut self,
        config: ExprTokenConfig,
        this: &FeatureRepr,
        field_ident: RangedCustomIdentifier,
        opt_associated_trace_id: Option<TraceId>,
    ) -> Vec<TraceTokenData> {
        match this {
            FeatureRepr::Expr(this) => {
                let mut tokens = self.feature_expr_tokens(this, config);
                tokens.extend([
                    special!("."),
                    ident!(field_ident.ident.as_str(), opt_associated_trace_id),
                ]);
                tokens
            }
            _ => vec![ident!(field_ident.ident.as_str())],
        }
    }

    fn feature_entity_call_tokens(
        &mut self,
        file: FilePtr,
        ranged_scope: RangedEntityRoute,
        inputs: &[Arc<FeatureLazyExpr>],
        opt_associated_trace_id: Option<TraceId>,
        config: ExprTokenConfig,
    ) -> Vec<TraceTokenData> {
        let text = self.eval_time().compile_time().text(file).unwrap();
        let mut tokens = vec![
            route!(text.ranged(ranged_scope.range), opt_associated_trace_id),
            special!("("),
        ];
        for (i, input) in inputs.iter().enumerate() {
            if i > 0 {
                tokens.push(special!(", "));
            }
            tokens.extend(self.feature_expr_tokens(input, config.subexpr()));
        }
        tokens.push(special!(")"));
        tokens
    }
}
