use entity_route::RangedEntityRoute;
use text::Text;
use vm::{History, StackValueSnapshot};

use super::{expr::ExprTokenConfig, *};
use crate::*;

impl<'eval> TraceFactory<'eval> {
    pub fn new_eager_expr_trace(
        &self,
        text: &Text,
        expr: Arc<EagerExpr>,
        history: Arc<History<'eval>>,
        opt_parent: Option<&Trace>,
        indent: Indent,
    ) -> Arc<Trace<'eval>> {
        self.new_trace(
            opt_parent.map(|parent| parent.id),
            indent,
            TraceVariant::EagerExpr { expr, history },
            text,
        )
    }

    pub(super) fn eager_expr_lines(
        &self,
        text: &Text,
        expr: &Arc<EagerExpr>,
        history: &Arc<History<'eval>>,
        indent: u8,
        config: ExprTokenConfig,
    ) -> Vec<LineProps<'eval>> {
        vec![LineProps {
            indent,
            idx: 0,
            tokens: self.eager_expr_tokens(expr, text, history, config),
        }]
    }

    pub(super) fn eager_expr_tokens(
        &self,
        expr: &Arc<EagerExpr>,
        text: &Text,
        history: &Arc<History<'eval>>,
        config: ExprTokenConfig,
    ) -> Vec<TokenProps<'eval>> {
        let associated_trace = if config.associated {
            Some(self.new_eager_expr_trace(text, expr.clone(), history.clone(), None, 0))
        } else {
            None
        };
        let mut tokens = vec![];
        match expr.variant {
            EagerExprVariant::Variable(ident) => tokens.push(ident!(ident.0, associated_trace)),
            EagerExprVariant::EntityRoute { route: scope } => todo!(),
            EagerExprVariant::PrimitiveLiteral(value) => return vec![literal!(value)],
            EagerExprVariant::Bracketed(_) => todo!(),
            EagerExprVariant::Opn {
                ref opn_variant,
                ref opds,
            } => match opn_variant {
                EagerOpnVariant::Binary { opr, this_ty: this } => {
                    tokens.extend(self.eager_expr_tokens(
                        &opds[0],
                        text,
                        history,
                        config.subexpr(),
                    ));
                    tokens.push(special!(opr.spaced_code(), associated_trace));
                    tokens.extend(self.eager_expr_tokens(
                        &opds[1],
                        text,
                        history,
                        config.subexpr(),
                    ));
                }
                EagerOpnVariant::Prefix { opr, .. } => {
                    tokens.push(special!(opr.code(), associated_trace));
                    tokens.extend(self.eager_expr_tokens(
                        &opds[0],
                        text,
                        history,
                        config.subexpr(),
                    ));
                }
                EagerOpnVariant::Suffix { opr, .. } => {
                    tokens.extend(self.eager_expr_tokens(
                        &opds[0],
                        text,
                        history,
                        config.subexpr(),
                    ));
                    tokens.push(special!(opr.code(), associated_trace));
                }
                EagerOpnVariant::RoutineCall(ranged_scope) => {
                    tokens = self.eager_routine_call_tokens(
                        *ranged_scope,
                        opds,
                        associated_trace,
                        text,
                        history,
                        &config,
                    )
                }
                EagerOpnVariant::PatternCall => todo!(),
                EagerOpnVariant::FieldAccess { .. } => todo!(),
                EagerOpnVariant::MethodCall { method_ident, .. } => {
                    tokens.extend(self.eager_expr_tokens(
                        &opds[0],
                        text,
                        history,
                        config.subexpr(),
                    ));
                    tokens.push(special!("."));
                    tokens.push(ident!(method_ident.ident.0));
                    tokens.push(special!("("));
                    for i in 1..opds.len() {
                        if i > 1 {
                            tokens.push(special!(", "))
                        }
                        tokens.extend(self.eager_expr_tokens(
                            &opds[i],
                            text,
                            history,
                            config.subexpr(),
                        ));
                    }
                    tokens.push(special!(")"));
                }
                EagerOpnVariant::ElementAccess => {
                    tokens.extend(self.eager_expr_tokens(
                        &opds[0],
                        text,
                        history,
                        config.subexpr(),
                    ));
                    tokens.push(special!("[", associated_trace.clone()));
                    for i in 1..opds.len() {
                        if i > 1 {
                            tokens.push(special!(", "))
                        }
                        tokens.extend(self.eager_expr_tokens(
                            &opds[i],
                            text,
                            history,
                            config.subexpr(),
                        ));
                    }
                    tokens.push(special!("]", associated_trace));
                }
                EagerOpnVariant::TypeCall { ranged_ty, .. } => {
                    tokens.push(scope!(text.ranged(ranged_ty.range)));
                    tokens.push(special!("("));
                    for i in 0..opds.len() {
                        if i > 0 {
                            tokens.push(special!(", "))
                        }
                        tokens.extend(self.eager_expr_tokens(
                            &opds[i],
                            text,
                            history,
                            config.subexpr(),
                        ));
                    }
                    tokens.push(special!(")"));
                }
            },
            EagerExprVariant::Lambda(_, _) => todo!(),
            EagerExprVariant::This => todo!(),
        };
        if config.appended {
            tokens.push(fade!(" = "));
            tokens.push(history.value(expr).into())
        }
        tokens
    }

    fn eager_routine_call_tokens(
        &self,
        ranged_scope: RangedEntityRoute,
        inputs: &[Arc<EagerExpr>],
        associated_trace: Option<Arc<Trace<'eval>>>,
        text: &Text,
        history: &Arc<History<'eval>>,
        config: &ExprTokenConfig,
    ) -> Vec<TokenProps<'eval>> {
        let mut tokens = vec![
            scope!(text.ranged(ranged_scope.range), associated_trace),
            special!("("),
        ];
        for (i, input) in inputs.iter().enumerate() {
            if i > 0 {
                tokens.push(special!(", "));
            }
            tokens.extend(self.eager_expr_tokens(input, text, history, config.subexpr()));
        }
        tokens.push(special!(")"));
        tokens
    }
}
