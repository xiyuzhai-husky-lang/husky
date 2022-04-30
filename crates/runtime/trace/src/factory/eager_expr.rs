use text::Text;
use vm::{History, StackValueSnapshot};

use super::{expr::ExprTokenConfig, *};
use crate::*;

impl<'eval> TraceFactory<'eval> {
    pub fn new_eager_expr_trace(
        &self,
        parent_id: TraceId,
        expr: Arc<EagerExpr>,
        value: StackValueSnapshot,
        history: Arc<History<'eval>>,
        text: &Text,
    ) -> Arc<Trace<'eval>> {
        self.new_trace(
            Some(parent_id),
            0,
            TraceVariant::EagerExpr { expr, history },
            text,
        )
    }

    pub(super) fn eager_expr_lines(
        &self,
        expr: &Arc<EagerExpr>,
        text: &Text,
        history: &Arc<History<'eval>>,
        config: ExprTokenConfig,
    ) -> Vec<LineProps<'eval>> {
        vec![LineProps {
            indent: 0,
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
            Some(self.new_trace(
                None,
                0,
                TraceVariant::EagerExpr {
                    expr: expr.clone(),
                    history: history.clone(),
                },
                text,
            ))
        } else {
            None
        };
        match expr.variant {
            EagerExprVariant::Variable(ident) => vec![ident!(ident.0, associated_trace)],
            EagerExprVariant::Scope { scope } => todo!(),
            EagerExprVariant::PrimitiveLiteral(value) => vec![literal!(value)],
            EagerExprVariant::Bracketed(_) => todo!(),
            EagerExprVariant::Opn {
                opn_variant: ref opn_kind,
                ref opds,
            } => {
                let mut tokens = vec![];
                match opn_kind {
                    EagerOpnVariant::Binary { opr, this } => {
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
                    EagerOpnVariant::Prefix { .. } => todo!(),
                    EagerOpnVariant::Suffix { .. } => todo!(),
                    EagerOpnVariant::RoutineCall(_) => todo!(),
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
                    EagerOpnVariant::ElementAccess => todo!(),
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
                }
                if config.appended {
                    tokens.push(fade!(" = "));
                    tokens.push(history.entry(expr).value().into())
                }
                tokens
            }
            EagerExprVariant::Lambda(_, _) => todo!(),
            EagerExprVariant::This => todo!(),
        }
    }
}
