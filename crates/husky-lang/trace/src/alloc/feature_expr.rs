use feature::{FeatureExpr, FeatureExprKind};

use crate::*;

impl TraceAllocator {
    pub fn feature_expr_trace(&self, parent: &Trace, expr: Arc<FeatureExpr>) -> Arc<Trace> {
        self.new_trace(
            Some(parent),
            parent.indent + 4,
            TraceKind::FeatureExpr(expr),
        )
    }

    pub fn feature_expr_subtraces(
        &self,
        parent: &Trace,
        expr: &FeatureExpr,
    ) -> Arc<Vec<Arc<Trace>>> {
        Arc::new(match expr.kind {
            FeatureExprKind::Literal(_) => vec![],
            FeatureExprKind::PrimitiveBinaryOpr {
                ref lopd, ref ropd, ..
            } => vec![
                self.feature_expr_trace(parent, lopd.clone()),
                self.feature_expr_trace(parent, ropd.clone()),
            ],
            FeatureExprKind::Variable { varname, ref value } => vec![],
        })
    }

    pub(crate) fn feature_expr_tokens(
        &self,
        expr: &Arc<FeatureExpr>,
        associated_with_trace: bool,
    ) -> Vec<TokenProps> {
        let associated_trace = if associated_with_trace {
            Some(self.new_trace(None, 0, TraceKind::FeatureExpr(expr.clone())))
        } else {
            None
        };
        match expr.kind {
            FeatureExprKind::Literal(value) => vec![literal!(value)],
            FeatureExprKind::PrimitiveBinaryOpr {
                opr,
                ref lopd,
                ref ropd,
            } => {
                let mut tokens = vec![];
                tokens.extend(self.feature_expr_tokens(lopd, associated_with_trace));
                tokens.push(special!(opr.spaced_code(), associated_trace));
                tokens.extend(self.feature_expr_tokens(ropd, associated_with_trace));
                tokens
            }
            FeatureExprKind::Variable { varname, .. } => vec![ident!(varname.0, associated_trace)],
        }
    }
}
