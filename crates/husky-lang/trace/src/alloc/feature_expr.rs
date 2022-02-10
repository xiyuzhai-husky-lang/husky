use feature::{FeatureExpr, FeatureExprKind};

use crate::*;

impl TraceAllocator {
    pub fn feature_expr_trace(
        &self,
        parent: usize,
        parent_indent: Indent,
        expr: Arc<FeatureExpr>,
    ) -> Arc<Trace> {
        self.new_trace(Some(parent), parent_indent + 4, TraceKind::Expr(expr))
    }

    pub fn feature_expr_subtraces(
        &self,
        parent: usize,
        parent_indent: Indent,
        expr: &FeatureExpr,
    ) -> Arc<Vec<Arc<Trace>>> {
        Arc::new(match expr.kind {
            FeatureExprKind::Literal(_) => vec![],
            FeatureExprKind::PrimitiveBinaryOpr {
                ref lopd, ref ropd, ..
            } => vec![
                self.feature_expr_trace(parent, parent_indent, lopd.clone()),
                self.feature_expr_trace(parent, parent_indent, ropd.clone()),
            ],
            FeatureExprKind::Variable { varname, ref value } => vec![],
        })
    }
}
