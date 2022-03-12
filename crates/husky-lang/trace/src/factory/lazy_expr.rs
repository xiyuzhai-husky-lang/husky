use feature::{LazyExpr, LazyExprKind};
use scope::RangedScope;

use super::expr::ExprTokenConfig;
use crate::*;

impl TraceFactory {
    pub(crate) fn lazy_expr_tokens(
        &self,
        expr: &Arc<LazyExpr>,
        text: &Text,
        config: ExprTokenConfig,
    ) -> Vec<TokenProps> {
        let associated_trace = if config.associated {
            Some(self.new_trace(None, 0, TraceKind::LazyExpr(expr.clone()), text))
        } else {
            None
        };
        return match expr.kind {
            LazyExprKind::Literal(value) => vec![literal!(value)],
            LazyExprKind::PrimitiveBinaryOpr {
                opr,
                ref lopd,
                ref ropd,
            } => {
                let mut tokens = vec![];
                tokens.extend(self.lazy_expr_tokens(lopd, text, config.subexpr()));
                tokens.push(special!(opr.spaced_code(), associated_trace));
                tokens.extend(self.lazy_expr_tokens(ropd, text, config.subexpr()));
                tokens
            }
            LazyExprKind::Variable { varname, .. } => vec![ident!(varname.0, associated_trace)],
            LazyExprKind::FuncCall {
                ranged_scope,
                ref inputs,
                ..
            } => routine_call_tokens(ranged_scope, inputs, associated_trace, text),
            LazyExprKind::ProcCall {
                ranged_scope,
                ref inputs,
                ..
            } => routine_call_tokens(ranged_scope, inputs, associated_trace, text),
        };

        fn routine_call_tokens(
            ranged_scope: RangedScope,
            inputs: &[Arc<LazyExpr>],
            associated_trace: Option<Arc<Trace>>,
            text: &Text,
        ) -> Vec<TokenProps> {
            let mut tokens = vec![
                scope!(text.ranged(ranged_scope.range), associated_trace),
                special!("("),
            ];
            for input in inputs {
                todo!()
            }
            tokens.push(special!(")"));
            tokens
        }
    }
}
