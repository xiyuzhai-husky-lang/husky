use entity_route::RangedEntityRoute;
use semantics_lazy::{LazyExprVariant, LazyOpnKind};

use super::expr::ExprTokenConfig;
use crate::*;

impl<'eval> TraceFactory<'eval> {
    pub(crate) fn feature_expr_lines(
        &self,
        expr: &Arc<FeatureExpr>,
        text: &Text,
        config: ExprTokenConfig,
    ) -> Vec<LineProps<'eval>> {
        vec![LineProps {
            indent: 0,
            idx: 0,
            tokens: self.feature_expr_tokens(expr, text, config),
        }]
    }

    pub(crate) fn feature_expr_tokens(
        &self,
        expr: &Arc<FeatureExpr>,
        text: &Text,
        config: ExprTokenConfig,
    ) -> Vec<TokenProps<'eval>> {
        let associated_trace = if config.associated {
            Some(self.new_trace(None, 0, TraceVariant::FeatureExpr(expr.clone()), text))
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
                tokens.push(special!(opr.spaced_code(), associated_trace));
                tokens.extend(self.feature_expr_tokens(ropd, text, config.subexpr()));
                tokens
            }
            FeatureExprVariant::Variable { varname, .. } => {
                vec![ident!(varname.0, associated_trace)]
            }
            FeatureExprVariant::RoutineCall {
                opds: ref feature_opds,
                ..
            } => match expr.expr.variant {
                LazyExprVariant::Opn { opn_kind, ref opds } => match opn_kind {
                    LazyOpnKind::Binary { opr, this } => todo!(),
                    LazyOpnKind::Prefix(_) => todo!(),
                    LazyOpnKind::NormalRoutineCall(ranged_route) => self
                        .feature_routine_call_tokens(
                            ranged_route,
                            feature_opds,
                            associated_trace,
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
                        opt_output_binding,
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
            FeatureExprVariant::StructOriginalFieldAccess { .. } => todo!(),
            FeatureExprVariant::EnumKindLiteral { .. } => todo!(),
            FeatureExprVariant::EntityFeature { .. } => todo!(),
            FeatureExprVariant::NewRecord { ty, ref opds, .. } => todo!(),
            FeatureExprVariant::RecordOriginalFieldAccess {
                ref this,
                field_ident,
                ..
            } => todo!(),
            FeatureExprVariant::This { ref repr } => todo!(),
            FeatureExprVariant::GlobalInput => vec![keyword!("input")],
            FeatureExprVariant::PatternCall {} => todo!(),
            FeatureExprVariant::RecordDerivedFieldAccess { .. } => todo!(),
            FeatureExprVariant::ElementAccess { ref opds, .. } => {
                let mut tokens = vec![];
                tokens.extend(self.feature_expr_tokens(&opds[0], text, config.subexpr()));
                tokens.push(special!("[", associated_trace.clone()));
                for i in 1..opds.len() {
                    let index_opd = &opds[i];
                    tokens.extend(self.feature_expr_tokens(index_opd, text, config.subexpr()));
                }
                tokens.push(special!("]", associated_trace));
                tokens
            }
            FeatureExprVariant::StructDerivedFieldAccess {
                ref this,
                field_ident,
                ref repr,
            } => todo!(),
        };
    }

    fn feature_routine_call_tokens(
        &self,
        ranged_scope: RangedEntityRoute,
        inputs: &[Arc<FeatureExpr>],
        associated_trace: Option<Arc<Trace<'eval>>>,
        text: &Text,
        config: ExprTokenConfig,
    ) -> Vec<TokenProps<'eval>> {
        let mut tokens = vec![
            scope!(text.ranged(ranged_scope.range), associated_trace),
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
}
