use semantics::{Expr, Opn, StrictExprKind};
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
            StrictExprKind::Variable(ident) => vec![ident!(ident.0, associated_trace)],
            StrictExprKind::Scope { scope, compiled } => todo!(),
            StrictExprKind::Literal(value) => vec![literal!(value)],
            StrictExprKind::Bracketed(_) => todo!(),
            StrictExprKind::Opn {
                opn,
                compiled,
                ref opds,
            } => {
                let mut tokens = vec![];
                match opn {
                    Opn::Binary { opr, this, kind } => {
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
                    Opn::Prefix(_) => todo!(),
                    Opn::Suffix(_) => todo!(),
                    Opn::RoutineCall(_) => todo!(),
                    Opn::PattCall => todo!(),
                    Opn::MembVarAccess => todo!(),
                    Opn::MembFuncCall(_) => todo!(),
                    Opn::ElementAccess => todo!(),
                }
                if config.appended {
                    tokens.push(fade!(" = "));
                    tokens.push(history.entry(expr).value().into())
                }
                tokens
            }
            StrictExprKind::Lambda(_, _) => todo!(),
        }
    }
}
