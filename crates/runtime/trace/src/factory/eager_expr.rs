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
            TraceKind::EagerExpr { expr, history },
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
                TraceKind::EagerExpr {
                    expr: expr.clone(),
                    history: history.clone(),
                },
                text,
            ))
        } else {
            None
        };
        match expr.kind {
            EagerExprKind::Variable(ident) => vec![ident!(ident.0, associated_trace)],
            EagerExprKind::Scope { scope } => todo!(),
            EagerExprKind::PrimitiveLiteral(value) => vec![literal!(value)],
            EagerExprKind::Bracketed(_) => todo!(),
            EagerExprKind::Opn {
                ref opn_kind,
                ref opds,
            } => {
                let mut tokens = vec![];
                match opn_kind {
                    EagerOpnKind::Binary { opr, this } => {
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
                    EagerOpnKind::Prefix { .. } => todo!(),
                    EagerOpnKind::Suffix { .. } => todo!(),
                    EagerOpnKind::RoutineCall(_) => todo!(),
                    EagerOpnKind::PatternCall => todo!(),
                    EagerOpnKind::FieldAccess { .. } => todo!(),
                    EagerOpnKind::MethodCall { .. } => todo!(),
                    EagerOpnKind::ElementAccess => todo!(),
                    EagerOpnKind::TypeCall { .. } => todo!(),
                }
                if config.appended {
                    tokens.push(fade!(" = "));
                    tokens.push(history.entry(expr).value().into())
                }
                tokens
            }
            EagerExprKind::Lambda(_, _) => todo!(),
            EagerExprKind::This => todo!(),
        }
    }
}
