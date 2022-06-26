use super::*;

impl HuskyTraceTime {
    pub fn feature_branch_trace(
        &mut self,
        parent: &Trace,
        indent: Indent,
        branch: Arc<FeatureLazyBranch>,
    ) -> TraceId {
        self.new_trace(
            Some(parent.id()),
            indent,
            TraceVariant::FeatureLazyBranch(branch),
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
    pub(crate) fn new_proc_stmt_trace(
        &mut self,
        parent_id: TraceId,
        indent: Indent,
        stmt: Arc<ProcStmt>,
        history: Arc<History<'static>>,
    ) -> TraceId {
        self.new_trace(
            Some(parent_id),
            indent,
            TraceVariant::ProcStmt { stmt, history },
        )
    }

    pub(crate) fn new_proc_branch_trace(
        &mut self,
        parent_id: TraceId,
        indent: Indent,
        stmt: Arc<ProcStmt>,
        branch: Arc<ProcConditionBranch>,
        branch_idx: u8,
        history: Arc<History<'static>>,
    ) -> TraceId {
        let opt_vm_branch = history.get(&stmt).map(|entry| match entry {
            HistoryEntry::ControlFlow { vm_branches, .. } => {
                vm_branches[branch_idx as usize].clone()
            }
            _ => panic!(),
        });
        self.new_trace(
            Some(parent_id),
            indent,
            TraceVariant::ProcBranch {
                stmt,
                branch,
                branch_idx,
                opt_vm_branch,
                history,
            },
        )
    }

    pub fn proc_stmts_traces(
        &mut self,
        parent_id: TraceId,
        indent: Indent,
        stmts: &[Arc<ProcStmt>],
        history: &Arc<History<'static>>,
    ) -> Vec<TraceId> {
        let mut traces = Vec::new();
        for stmt in stmts {
            match stmt.variant {
                ProcStmtVariant::ConditionFlow { ref branches } => {
                    for (branch_idx, branch) in branches.iter().enumerate() {
                        traces.push(self.new_proc_branch_trace(
                            parent_id,
                            indent,
                            stmt.clone(),
                            branch.clone(),
                            branch_idx.try_into().unwrap(),
                            history.clone(),
                        ))
                    }
                }
                _ => traces.push(self.new_proc_stmt_trace(
                    parent_id,
                    indent,
                    stmt.clone(),
                    history.clone(),
                )),
            }
        }
        traces
    }
}
