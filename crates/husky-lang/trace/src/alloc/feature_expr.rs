use feature::{FeatureExpr, FeatureExprKind};
use scope::RangedScope;

use crate::*;

impl TraceAllocator {
    pub(crate) fn feature_expr_associated_tokens(
        &self,
        indent: Indent,
        expr: &Arc<FeatureExpr>,
        text: &Text,
    ) -> Vec<TokenProps> {
        let associated_trace =
            self.new_trace(None, indent, TraceKind::FeatureExpr(expr.clone()), text);
        return match expr.kind {
            FeatureExprKind::Literal(value) => vec![literal!(value)],
            FeatureExprKind::PrimitiveBinaryOpr {
                opr,
                ref lopd,
                ref ropd,
            } => {
                let mut tokens = vec![];
                tokens.extend(self.feature_expr_associated_tokens(indent, lopd, text));
                tokens.push(special!(opr.spaced_code(), associated_trace));
                tokens.extend(self.feature_expr_associated_tokens(indent, ropd, text));
                tokens
            }
            FeatureExprKind::Variable { varname, .. } => vec![ident!(varname.0, associated_trace)],
            FeatureExprKind::FuncCall {
                ranged_scope,
                ref inputs,
                ..
            } => routine_call_tokens(ranged_scope, inputs, associated_trace, text),
            FeatureExprKind::ProcCall {
                ranged_scope,
                ref inputs,
                ..
            } => routine_call_tokens(ranged_scope, inputs, associated_trace, text),
        };

        fn routine_call_tokens(
            ranged_scope: RangedScope,
            inputs: &[Arc<FeatureExpr>],
            associated_trace: Arc<Trace>,
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

    pub(crate) fn feature_expr_tokens(
        &self,
        expr: &Arc<FeatureExpr>,
        text: &Text,
    ) -> Vec<TokenProps> {
        match expr.kind {
            FeatureExprKind::Literal(value) => vec![literal!(value)],
            FeatureExprKind::PrimitiveBinaryOpr {
                opr,
                ref lopd,
                ref ropd,
            } => {
                let mut tokens = vec![];
                tokens.extend(self.feature_expr_tokens(lopd, text));
                tokens.push(special!(opr.spaced_code()));
                tokens.extend(self.feature_expr_tokens(ropd, text));
                tokens
            }
            FeatureExprKind::Variable { varname, .. } => vec![ident!(varname.0)],
            FeatureExprKind::FuncCall {
                ranged_scope,
                ref inputs,
                ..
            } => {
                let mut tokens = vec![scope!(text.ranged(ranged_scope.range)), special!("(")];
                for input in inputs {
                    todo!()
                }
                tokens.push(special!(")"));
                tokens
            }
            FeatureExprKind::ProcCall {
                ranged_scope,
                ref inputs,
                ..
            } => {
                let mut tokens = vec![scope!(text.ranged(ranged_scope.range)), special!("(")];
                for input in inputs {
                    todo!()
                }
                tokens.push(special!(")"));
                tokens
            }
        }
    }
}
