use feature::{FeatureExpr, FeatureExprKind};

use crate::*;

pub fn eval_feature_expr_trace(
    parent: usize,
    parent_indent: Indent,
    expr: Arc<FeatureExpr>,
) -> Arc<Trace> {
    Trace::new(Some(parent), parent_indent + 4, TraceKind::Expr(expr))
}

pub fn eval_feature_expr_subtraces(
    parent: usize,
    parent_indent: Indent,
    expr: &FeatureExpr,
) -> Arc<Vec<Arc<Trace>>> {
    Arc::new(match expr.kind {
        FeatureExprKind::Literal(_) => vec![],
        FeatureExprKind::PrimitiveBinaryOpr {
            ref lopd, ref ropd, ..
        } => vec![
            eval_feature_expr_trace(parent, parent_indent, lopd.clone()),
            eval_feature_expr_trace(parent, parent_indent, ropd.clone()),
        ],
        FeatureExprKind::Variable { varname, ref value } => vec![],
    })
}

pub fn eval_feature_expr_trace_tokens(expr: &FeatureExpr) -> Vec<TokenProps> {
    match expr.kind {
        FeatureExprKind::Literal(value) => vec![literal!(value)],
        FeatureExprKind::PrimitiveBinaryOpr {
            opr,
            ref lopd,
            ref ropd,
        } => {
            let mut tokens = vec![];
            tokens.extend(eval_feature_expr_trace_tokens(&lopd));
            tokens.push(special!(opr.code()));
            tokens.extend(eval_feature_expr_trace_tokens(&ropd));
            tokens
        }
        FeatureExprKind::Variable { varname, .. } => vec![ident!(varname.0)],
    }
}
