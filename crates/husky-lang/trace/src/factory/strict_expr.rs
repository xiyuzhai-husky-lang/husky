use semantics::{Expr, ExprKind, OpnKind};
use text::Text;
use vm::{History, StackValueSnapshot};

use super::{expr::ExprTokenConfig, *};
use crate::*;

impl TraceFactory {
    pub fn new_expr_trace(
        &self,
        parent_id: TraceId,
        expr: Arc<Expr>,
        value: StackValueSnapshot,
        history: Arc<History>,
        text: &Text,
    ) -> Arc<Trace> {
        self.new_trace(
            Some(parent_id),
            0,
            TraceKind::StrictExpr { expr, history },
            text,
        )
    }

    pub(super) fn strict_expr_lines(
        &self,
        expr: &Arc<Expr>,
        text: &Text,
        history: &Arc<History>,
        config: ExprTokenConfig,
    ) -> Vec<LineProps> {
        vec![LineProps {
            indent: 0,
            idx: 0,
            tokens: self.strict_expr_tokens(expr, text, history, config),
        }]
    }

    pub(super) fn strict_expr_tokens(
        &self,
        expr: &Arc<Expr>,
        text: &Text,
        history: &Arc<History>,
        config: ExprTokenConfig,
    ) -> Vec<TokenProps> {
        let associated_trace = if config.associated {
            Some(self.new_trace(
                None,
                0,
                TraceKind::StrictExpr {
                    expr: expr.clone(),
                    history: history.clone(),
                },
                text,
            ))
        } else {
            None
        };
        match expr.kind {
            ExprKind::Variable(ident) => vec![ident!(ident.0, associated_trace)],
            ExprKind::Scope { scope, compiled } => todo!(),
            ExprKind::Literal(value) => vec![literal!(value)],
            ExprKind::Bracketed(_) => todo!(),
            ExprKind::Opn {
                opn_kind: opn,
                compiled,
                ref opds,
            } => {
                let mut tokens = vec![];
                match opn {
                    OpnKind::Binary { opr, this } => {
                        tokens.extend(self.strict_expr_tokens(
                            &opds[0],
                            text,
                            history,
                            config.subexpr(),
                        ));
                        tokens.push(special!(opr.spaced_code(), associated_trace));
                        tokens.extend(self.strict_expr_tokens(
                            &opds[1],
                            text,
                            history,
                            config.subexpr(),
                        ));
                    }
                    OpnKind::Prefix(_) => todo!(),
                    OpnKind::Suffix(_) => todo!(),
                    OpnKind::RoutineCall(_) => todo!(),
                    OpnKind::PattCall => todo!(),
                    OpnKind::MembVarAccess => todo!(),
                    OpnKind::MembFuncCall(_) => todo!(),
                    OpnKind::ElementAccess => todo!(),
                }
                if config.appended {
                    tokens.push(fade!(" = "));
                    tokens.push(history.entry(expr).value().into())
                }
                tokens
            }
            ExprKind::Lambda(_, _) => todo!(),
        }
    }
}
