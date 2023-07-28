use super::*;

impl Debugtime {
    pub(crate) fn new_proc_stmt_trace(
        &mut self,
        parent_id: TraceId,
        indent: Indent,
        stmt: Arc<ProcStmt>,
        history: Arc<History>,
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
        branch: Arc<ProcConditionFlowBranch>,
        branch_idx: u8,
        history: Arc<History>,
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

    pub(crate) fn proc_stmts_traces(
        &mut self,
        parent_id: TraceId,
        indent: Indent,
        stmts: &[Arc<ProcStmt>],
        history: &Arc<History>,
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
