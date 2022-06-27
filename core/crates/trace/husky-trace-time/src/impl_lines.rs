use super::*;

impl HuskyTraceTime {
    pub(crate) fn feature_expr_lines(
        &mut self,
        expr: &Arc<FeatureExpr>,
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
        branch: &FeatureLazyBranch,
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

    pub(crate) fn proc_stmt_lines(
        &mut self,
        stmt: &ProcStmt,
        history: &Arc<History<'static>>,
    ) -> Vec<TraceLineData> {
        vec![TraceLineData {
            indent: stmt.indent,
            tokens: self.proc_stmt_tokens(stmt, history),
            idx: 0,
        }]
    }

    pub(crate) fn loop_frame_lines(
        &self,
        indent: Indent,
        loop_frame_data: &LoopFrameData,
    ) -> Vec<TraceLineData> {
        vec![TraceLineData {
            indent,
            tokens: self.loop_frame_tokens(loop_frame_data),
            idx: 0,
        }]
    }

    pub(crate) fn proc_branch_lines(
        &mut self,
        indent: Indent,
        branch: &ProcConditionBranch,
        history: &Arc<History<'static>>,
    ) -> Vec<TraceLineData> {
        vec![TraceLineData {
            indent,
            tokens: self.proc_branch_tokens(indent, branch, history),
            idx: 0,
        }]
    }

    pub(crate) fn eager_expr_lines(
        &mut self,
        expr: &Arc<EagerExpr>,
        history: &Arc<History<'static>>,
        indent: u8,
        config: ExprTokenConfig,
    ) -> Vec<TraceLineData> {
        vec![TraceLineData {
            indent,
            idx: 0,
            tokens: self.eager_expr_tokens(expr, history, config),
        }]
    }
}
