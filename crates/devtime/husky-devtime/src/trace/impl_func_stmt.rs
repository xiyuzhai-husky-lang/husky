use husky_syn_expr::SynStmtIdxRange;

use super::*;

impl<Task: IsTask> Devtime<Task> {
    // pub(crate) fn func_stmts_traces(
    //     &mut self,
    //     parent_id: TraceId,
    //     indent: Indent,
    //     stmts: SynStmtIdxRange,
    //     history: &Arc<History>,
    // ) -> Vec<TraceId> {
    //     let mut traces = Vec::new();
    //     for stmt in stmts {
    //         match stmt.variant {
    //             FuncStmtVariant::ConditionFlow { ref branches } => {
    //                 for (branch_idx, branch) in branches.iter().enumerate() {
    //                     traces.push(self.new_func_branch_trace(
    //                         parent_id,
    //                         indent,
    //                         stmt.clone(),
    //                         branch.clone(),
    //                         branch_idx.try_into().unwrap(),
    //                         history.clone(),
    //                     ))
    //                 }
    //             }
    //             _ => traces.push(self.new_func_stmt_trace(
    //                 parent_id,
    //                 indent,
    //                 stmt.clone(),
    //                 history.clone(),
    //             )),
    //         }
    //     }
    //     traces
    // }

    // pub(crate) fn new_func_branch_trace(
    //     &mut self,
    //     parent_id: TraceId,
    //     indent: Indent,
    //     stmt: SynStmtIdx,
    //     branch: Arc<FuncConditionFlowBranch>,
    //     branch_idx: u8,
    //     history: Arc<History>,
    // ) -> TraceId {
    //     let opt_vm_branch = history.get(&stmt).map(|entry| match entry {
    //         HistoryEntry::ControlFlow { vm_branches, .. } => {
    //             vm_branches[branch_idx as usize].clone()
    //         }
    //         _ => panic!(),
    //     });
    //     self.new_trace(
    //         Some(parent_id),
    //         indent,
    //         TraceVariant::FuncBranch {
    //             stmt,
    //             branch,
    //             branch_idx,
    //             opt_vm_branch,
    //             history,
    //         },
    //     )
    // }

    // pub(crate) fn new_func_stmt_trace(
    //     &mut self,
    //     parent_id: TraceId,
    //     indent: Indent,
    //     stmt: SynStmtIdx,
    //     history: Arc<History>,
    // ) -> TraceId {
    //     self.new_trace(
    //         Some(parent_id),
    //         indent,
    //         TraceVariant::FuncStmt { stmt, history },
    //     )
    // }
}
