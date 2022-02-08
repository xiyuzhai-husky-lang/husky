use feature::FeatureStmt;

use crate::*;

use super::*;

pub fn eval_feature_stmt_trace(parent: Option<usize>, stmt: Arc<FeatureStmt>) -> Arc<Trace> {
    Trace::new(parent, stmt.indent, TraceKind::Stmt(stmt))
}

pub fn eval_feature_stmt_subtraces(parent: usize, stmt: &FeatureStmt) -> Arc<Vec<Arc<Trace>>> {
    Arc::new(match stmt.kind {
        feature::FeatureStmtKind::Init { varname, ref value } => {
            vec![eval_feature_expr_trace(parent, stmt.indent, value.clone())]
        }
        feature::FeatureStmtKind::Assert { ref condition } => {
            vec![eval_feature_expr_trace(
                parent,
                stmt.indent,
                condition.clone(),
            )]
        }
        feature::FeatureStmtKind::Return { ref result } => {
            vec![eval_feature_expr_trace(parent, stmt.indent, result.clone())]
        }
    })
}

pub fn eval_feature_stmt_trace_tokens(stmt: &FeatureStmt) -> Vec<TokenProps> {
    match stmt.kind {
        feature::FeatureStmtKind::Init { varname, ref value } => {
            let mut tokens = vec![];
            tokens.push(ident!(varname.0, 0));
            tokens.push(special!("="));
            tokens.extend(eval_feature_expr_trace_tokens(value));
            tokens.into()
        }
        feature::FeatureStmtKind::Assert { ref condition } => {
            let mut tokens = vec![];
            tokens.push(keyword!("assert", 0));
            tokens.extend(eval_feature_expr_trace_tokens(condition));
            tokens.into()
        }
        feature::FeatureStmtKind::Return { ref result } => {
            let mut tokens = vec![];
            tokens.extend(eval_feature_expr_trace_tokens(result));
            tokens[0].spaces_before = Some(0);
            tokens.into()
        }
    }
}
