use feature::{FeatureBranch, FeatureExpr, FeatureExprKind, FeatureStmt, FeatureStmtKind};

use crate::*;

pub(crate) fn feature_stmt_trace_tokens(stmt: &FeatureStmt) -> Vec<TokenProps> {
    match stmt.kind {
        FeatureStmtKind::Init { varname, ref value } => {
            let mut tokens = vec![];
            tokens.push(ident!(varname.0, 0));
            tokens.push(special!("="));
            tokens.extend(feature_expr_trace_tokens(value));
            tokens.into()
        }
        FeatureStmtKind::Assert { ref condition } => {
            let mut tokens = vec![];
            tokens.push(keyword!("assert", 0));
            tokens.extend(feature_expr_trace_tokens(condition));
            tokens.into()
        }
        FeatureStmtKind::Return { ref result } => {
            let mut tokens = vec![];
            tokens.extend(feature_expr_trace_tokens(result));
            tokens[0].spaces_before = Some(0);
            tokens.into()
        }
        FeatureStmtKind::Branches { .. } => vec![keyword!("if", 0), special!("...")],
    }
}

pub(crate) fn feature_expr_trace_tokens(expr: &FeatureExpr) -> Vec<TokenProps> {
    match expr.kind {
        FeatureExprKind::Literal(value) => vec![literal!(value)],
        FeatureExprKind::PrimitiveBinaryOpr {
            opr,
            ref lopd,
            ref ropd,
        } => {
            let mut tokens = vec![];
            tokens.extend(feature_expr_trace_tokens(&lopd));
            tokens.push(special!(opr.code()));
            tokens.extend(feature_expr_trace_tokens(&ropd));
            tokens
        }
        FeatureExprKind::Variable { varname, .. } => vec![ident!(varname.0)],
    }
}

pub(crate) fn feature_branch_trace_tokens(branch: &FeatureBranch) -> Vec<TokenProps> {
    match branch.kind {
        feature::FeatureBranchKind::If { ref condition } => {
            let mut tokens = vec![keyword!("if", 0)];
            tokens.extend(feature_expr_trace_tokens(condition));
            tokens
        }
        feature::FeatureBranchKind::Elif { ref condition } => {
            let mut tokens = vec![keyword!("elif", 0)];
            tokens.extend(feature_expr_trace_tokens(condition));
            tokens
        }
        feature::FeatureBranchKind::Else => vec![keyword!("else", 0)],
    }
}
