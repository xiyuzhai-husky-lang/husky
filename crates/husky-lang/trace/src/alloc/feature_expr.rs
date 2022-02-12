use feature::{FeatureExpr, FeatureExprKind};

use crate::*;

impl TraceAllocator {
    pub fn feature_expr_subtraces(
        &self,
        parent: &Trace,
        expr: &FeatureExpr,
    ) -> Arc<Vec<Arc<Trace>>> {
        msg_once!("todo: feature_expr_subtraces");
        Arc::new(Vec::new())
        // Arc::new(match expr.kind {
        //     FeatureExprKind::Literal(_) => vec![],
        //     FeatureExprKind::PrimitiveBinaryOpr {
        //         ref lopd, ref ropd, ..
        //     } => vec![
        //         self.feature_expr_trace(parent, lopd.clone()),
        //         self.feature_expr_trace(parent, ropd.clone()),
        //     ],
        //     FeatureExprKind::Variable { varname, ref value } => vec![],
        // })
    }

    pub(crate) fn feature_expr_associated_tokens(
        &self,
        indent: Indent,
        expr: &Arc<FeatureExpr>,
    ) -> Vec<TokenProps> {
        let associated_trace =
            Some(self.new_trace(None, indent, TraceKind::FeatureExpr(expr.clone())));
        match expr.kind {
            FeatureExprKind::Literal(value) => vec![literal!(value)],
            FeatureExprKind::PrimitiveBinaryOpr {
                opr,
                ref lopd,
                ref ropd,
            } => {
                let mut tokens = vec![];
                tokens.extend(self.feature_expr_associated_tokens(indent, lopd));
                tokens.push(special!(opr.spaced_code(), associated_trace));
                tokens.extend(self.feature_expr_associated_tokens(indent, ropd));
                tokens
            }
            FeatureExprKind::Variable { varname, .. } => vec![ident!(varname.0, associated_trace)],
        }
    }

    pub(crate) fn feature_expr_tokens(&self, expr: &Arc<FeatureExpr>) -> Vec<TokenProps> {
        match expr.kind {
            FeatureExprKind::Literal(value) => vec![literal!(value)],
            FeatureExprKind::PrimitiveBinaryOpr {
                opr,
                ref lopd,
                ref ropd,
            } => {
                let mut tokens = vec![];
                tokens.extend(self.feature_expr_tokens(lopd));
                tokens.push(special!(opr.spaced_code(), None));
                tokens.extend(self.feature_expr_tokens(ropd));
                tokens
            }
            FeatureExprKind::Variable { varname, .. } => vec![ident!(varname.0, None)],
        }
    }
}
