use super::*;

impl HuskyTraceTime {
    pub(crate) fn feature_expr_lines(
        &mut self,
        expr: &Arc<FeatureLazyExpr>,
        config: ExprTokenConfig,
    ) -> Vec<TraceLineData> {
        vec![TraceLineData {
            indent: 0,
            idx: 0,
            tokens: self.feature_expr_tokens(expr, config),
        }]
    }
    pub(crate) fn feature_branch_lines(
        &mut self,
        indent: Indent,
        branch: &FeatureBranch,
    ) -> Vec<TraceLineData> {
        vec![TraceLineData {
            idx: 0,
            indent,
            tokens: self.feature_branch_tokens(branch),
        }]
    }
    pub(crate) fn func_stmt_lines(
        &mut self,
        stmt: &FuncStmt,
        history: &Arc<History<'static>>,
    ) -> Vec<TraceLineData> {
        vec![TraceLineData {
            indent: stmt.indent,
            tokens: self.func_stmt_tokens(stmt, history),
            idx: 0,
        }]
    }
}
