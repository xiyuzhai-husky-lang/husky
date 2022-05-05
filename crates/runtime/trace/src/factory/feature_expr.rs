use entity_route::RangedEntityRoute;
use semantics_lazy::{LazyExprKind, LazyOpnKind};

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
        return match expr.kind {
            FeatureExprKind::PrimitiveLiteral(value) => vec![literal!(value)],
            FeatureExprKind::PrimitiveBinaryOpr {
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
            FeatureExprKind::Variable { varname, .. } => vec![ident!(varname.0, associated_trace)],
            FeatureExprKind::RoutineCall {
                opds: ref feature_opds,
                ..
            } => match expr.expr.kind {
                LazyExprKind::Opn { opn_kind, ref opds } => match opn_kind {
                    LazyOpnKind::Binary { opr, this } => todo!(),
                    LazyOpnKind::Prefix(_) => todo!(),
                    LazyOpnKind::RoutineCall(ranged_route) => self.routine_call_tokens(
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
                    } => todo!(),
                    LazyOpnKind::MethodCall {
                        method_ident,
                        method_route,
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
                    LazyOpnKind::ElementAccess => todo!(),
                },
                _ => panic!(""),
            },
            FeatureExprKind::StructOriginalFieldAccess { .. } => todo!(),
            FeatureExprKind::EnumLiteral { .. } => todo!(),
            FeatureExprKind::EntityFeature { .. } => todo!(),
            FeatureExprKind::NewRecord { ty, ref opds, .. } => todo!(),
            FeatureExprKind::RecordOriginalFieldAccess {
                ref this,
                field_ident,
                ..
            } => todo!(),
            FeatureExprKind::This { ref repr } => todo!(),
            FeatureExprKind::GlobalInput => vec![keyword!("input")],
            FeatureExprKind::PatternCall {} => todo!(),
            FeatureExprKind::RecordDerivedFieldAccess { .. } => todo!(),
            FeatureExprKind::ElementAccess { ref opds, .. } => {
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
        };
    }

    fn routine_call_tokens(
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
