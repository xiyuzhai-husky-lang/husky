use feature::{FeatureExpr, FeatureExprKind};

use crate::*;

impl TraceAllocator {
    pub fn feature_expr_subtraces(
        &self,
        parent: &Trace,
        expr: &FeatureExpr,
        maybe_locked_on: Option<usize>,
    ) -> Arc<Vec<Arc<Trace>>> {
        Arc::new(match expr.kind {
            FeatureExprKind::Literal(_)
            | FeatureExprKind::PrimitiveBinaryOpr { .. }
            | FeatureExprKind::Variable { .. } => vec![],
            FeatureExprKind::FuncCall { ref inputs, .. } => {
                if let Some(locked_on) = maybe_locked_on {
                    let mut subtraces = vec![];
                    for input in inputs {
                        todo!("add input trace")
                    }
                    todo!();
                    subtraces
                } else {
                    vec![]
                }
            }
        })
    }

    pub(crate) fn feature_expr_associated_tokens(
        &self,
        indent: Indent,
        expr: &Arc<FeatureExpr>,
        text: &Text,
    ) -> Vec<TokenProps> {
        let associated_trace =
            Some(self.new_trace(None, indent, TraceKind::FeatureExpr(expr.clone()), text));
        match expr.kind {
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
                func,
                scope_expr_range,
                ref inputs,
                ..
            } => {
                let mut tokens = vec![
                    scope!(text.ranged(scope_expr_range), associated_trace),
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
            FeatureExprKind::Variable { varname, .. } => vec![ident!(varname.0, None)],
            FeatureExprKind::FuncCall {
                func,
                scope_expr_range,
                ref inputs,
                ..
            } => {
                let mut tokens = vec![scope!(text.ranged(scope_expr_range), None), special!("(")];
                for input in inputs {
                    todo!()
                }
                tokens.push(special!(")"));
                tokens
            }
        }
    }
}
