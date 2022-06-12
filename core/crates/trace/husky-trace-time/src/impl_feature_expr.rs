use super::impl_expr::ExprTokenConfig;
use crate::*;
use entity_route::RangedEntityRoute;
use eval_feature::EvalFeature;
use semantics_lazy::{LazyExprVariant, LazyOpnKind};
use text::RangedCustomIdentifier;
use vm::InterpreterQueryGroup;
use word::CustomIdentifier;

impl HuskyTraceTime {
    pub(crate) fn feature_expr_lines(
        &mut self,
        expr: &Arc<FeatureExpr>,
        text: &Text,
        config: ExprTokenConfig,
    ) -> Vec<TraceLineData> {
        vec![TraceLineData {
            indent: 0,
            idx: 0,
            tokens: self.feature_expr_tokens(expr, text, config),
        }]
    }

    pub(crate) fn feature_expr_tokens(
        &mut self,
        expr: &Arc<FeatureExpr>,
        text: &Text,
        config: ExprTokenConfig,
    ) -> Vec<TraceTokenData> {
        let opt_associated_trace_id = if config.associated {
            Some(
                self.new_trace(None, 0, TraceVariant::FeatureExpr(expr.clone()), text)
                    .id(),
            )
        } else {
            None
        };
        return match expr.variant {
            FeatureExprVariant::PrimitiveLiteral(value) => vec![literal!(value)],
            FeatureExprVariant::PrimitiveBinaryOpr {
                opr,
                ref lopd,
                ref ropd,
            } => {
                let mut tokens = vec![];
                tokens.extend(self.feature_expr_tokens(lopd, text, config.subexpr()));
                tokens.push(special!(opr.spaced_code(), opt_associated_trace_id));
                tokens.extend(self.feature_expr_tokens(ropd, text, config.subexpr()));
                tokens
            }
            FeatureExprVariant::Variable { varname, .. } => {
                vec![ident!(varname.0, opt_associated_trace_id)]
            }
            FeatureExprVariant::RoutineCall {
                opds: ref feature_opds,
                ..
            } => match expr.expr.variant {
                LazyExprVariant::Opn { opn_kind, ref opds } => match opn_kind {
                    LazyOpnKind::Binary { opr, this } => todo!(),
                    LazyOpnKind::Prefix(_) => todo!(),
                    LazyOpnKind::FunctionRoutineCall(ranged_route) => self
                        .feature_routine_call_tokens(
                            ranged_route,
                            feature_opds,
                            opt_associated_trace_id,
                            text,
                            config,
                        ),
                    LazyOpnKind::StructCall(_) => todo!(),
                    LazyOpnKind::RecordCall(_) => todo!(),
                    LazyOpnKind::PatternCall => todo!(),
                    LazyOpnKind::FieldAccess {
                        field_ident,
                        field_kind,
                        ..
                    } => todo!(),
                    LazyOpnKind::MethodCall {
                        method_ident,
                        method_route,
                        output_binding,
                    } => {
                        let mut tokens = vec![];
                        tokens.extend(self.feature_expr_tokens(
                            &feature_opds[0],
                            text,
                            config.subexpr(),
                        ));
                        tokens.push(special!("."));
                        tokens.push(ident!(method_ident.ident.0));
                        tokens.push(special!("("));
                        for i in 1..opds.len() {
                            if i > 1 {
                                tokens.push(special!(", "))
                            }
                            tokens.extend(self.feature_expr_tokens(
                                &feature_opds[i],
                                text,
                                config.subexpr(),
                            ));
                        }
                        tokens.push(special!(")"));
                        tokens
                    }
                    LazyOpnKind::ElementAccess { .. } => todo!(),
                },
                _ => panic!(""),
            },
            FeatureExprVariant::EnumKindLiteral { .. } => todo!(),
            FeatureExprVariant::EntityFeature { .. } => {
                vec![route!(
                    text.ranged(expr.expr.range),
                    opt_associated_trace_id
                )]
            }
            FeatureExprVariant::NewRecord { ty, ref opds, .. } => todo!(),
            FeatureExprVariant::ThisValue { ref repr } => todo!(),
            FeatureExprVariant::EvalInput => vec![keyword!("input")],
            FeatureExprVariant::PatternCall {} => todo!(),
            FeatureExprVariant::ElementAccess { ref opds, .. } => {
                let mut tokens = vec![];
                tokens.extend(self.feature_expr_tokens(&opds[0], text, config.subexpr()));
                tokens.push(special!("[", opt_associated_trace_id.clone()));
                for i in 1..opds.len() {
                    let index_opd = &opds[i];
                    tokens.extend(self.feature_expr_tokens(index_opd, text, config.subexpr()));
                }
                tokens.push(special!("]", opt_associated_trace_id));
                tokens
            }
            FeatureExprVariant::RecordDerivedFieldAccess {
                ref this,
                field_ident,
                ..
            } => self.field_access_tokens(text, config, this, field_ident),
            FeatureExprVariant::StructOriginalFieldAccess {
                ref this,
                field_ident,
                ..
            } => self.field_access_tokens(text, config, this, field_ident),
            FeatureExprVariant::RecordOriginalFieldAccess {
                ref this,
                field_ident,
                ..
            } => self.field_access_tokens(text, config, this, field_ident),
            FeatureExprVariant::StructDerivedLazyFieldAccess {
                ref this,
                field_ident,
                ref repr,
            } => self.field_access_tokens(text, config, this, field_ident),
        };
    }

    fn field_access_tokens(
        &mut self,
        text: &Text,
        config: ExprTokenConfig,
        this: &Arc<FeatureExpr>,
        field_ident: RangedCustomIdentifier,
    ) -> Vec<TraceTokenData> {
        let mut tokens = self.feature_expr_tokens(this, text, config);
        tokens.extend([special!("."), ident!(field_ident.ident.as_str())]);
        tokens
    }

    fn feature_routine_call_tokens(
        &mut self,
        ranged_scope: RangedEntityRoute,
        inputs: &[Arc<FeatureExpr>],
        opt_associated_trace_id: Option<TraceId>,
        text: &Text,
        config: ExprTokenConfig,
    ) -> Vec<TraceTokenData> {
        let mut tokens = vec![
            route!(text.ranged(ranged_scope.range), opt_associated_trace_id),
            special!("("),
        ];
        for (i, input) in inputs.iter().enumerate() {
            if i > 0 {
                tokens.push(special!(", "));
            }
            tokens.extend(self.feature_expr_tokens(input, text, config.subexpr()));
        }
        tokens.push(special!(")"));
        tokens
    }

    pub(crate) fn feature_expr_figure(&self, expr: &FeatureExpr, focus: &Focus) -> FigureProps {
        match focus.opt_input_id {
            Some(input_id) => {
                if let Ok(value) = self.runtime.eval_feature_expr(expr, input_id) {
                    FigureProps::new_specific(
                        self.runtime.visualize(expr.expr.ty(), value.any_ref()),
                    )
                } else {
                    FigureProps::void()
                }
            }
            None => todo!(),
        }
    }
}
