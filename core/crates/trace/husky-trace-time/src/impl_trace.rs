use super::*;

impl HuskyTraceTime {
    pub fn feature_branch_trace(
        &mut self,
        parent: &Trace,
        indent: Indent,
        branch: Arc<FeatureBranch>,
    ) -> TraceId {
        self.new_trace(
            Some(parent.id()),
            indent,
            TraceVariant::FeatureBranch(branch),
        )
    }
    pub fn func_stmts_traces<'a>(
        &'a mut self,
        parent_id: TraceId,
        indent: Indent,
        stmts: &'a [Arc<FuncStmt>],
        history: &'a Arc<History<'static>>,
    ) -> impl Iterator<Item = TraceId> + 'a {
        stmts.iter().map(move |stmt| {
            self.new_func_stmt_trace(parent_id, indent, stmt.clone(), history.clone())
        })
    }
    pub fn new_func_stmt_trace(
        &mut self,
        parent_id: TraceId,
        indent: Indent,
        stmt: Arc<FuncStmt>,
        history: Arc<History<'static>>,
    ) -> TraceId {
        self.new_trace(
            Some(parent_id),
            indent,
            TraceVariant::FuncStmt { stmt, history },
        )
    }
}
