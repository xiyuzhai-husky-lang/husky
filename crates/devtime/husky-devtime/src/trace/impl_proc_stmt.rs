use husky_syn_expr::{SynStmtIdx, SynStmtIdxRange};

use super::*;

impl<Task: IsTask> Devtime<Task> {
    pub(crate) fn new_proc_stmt_trace(
        &mut self,
        parent_id: TraceId,
        indent: Indent,
        stmt: SynStmtIdx,
        history: Arc<History>,
    ) -> TraceId {
        todo!()
        // self.new_trace(
        //     Some(parent_id),
        //     indent,
        //     TraceVariant::EagerStmt { stmt, history },
        // )
    }

    pub(crate) fn new_proc_branch_trace(
        &mut self,
        parent_id: TraceId,
        indent: Indent,
        stmt: SynStmtIdx,
        // branch: Arc<ProcConditionFlowBranch>,
        branch_idx: u8,
        history: Arc<History>,
    ) -> TraceId {
        todo!()
        // let opt_vm_branch = history.get(&stmt).map(|entry| match entry {
        //     HistoryEntry::ControlFlow { vm_branches, .. } => {
        //         vm_branches[branch_idx as usize].clone()
        //     }
        //     _ => panic!(),
        // });
        // self.new_trace(
        //     Some(parent_id),
        //     indent,
        //     TraceVariant::EagerBranch {
        //         stmt,
        //         branch,
        //         branch_idx,
        //         opt_vm_branch,
        //         history,
        //     },
        // )
    }

    pub(crate) fn proc_stmts_traces(
        &mut self,
        parent_id: TraceId,
        // indent: Indent,
        stmts: SynStmtIdxRange,
        history: &Arc<History>,
    ) -> Vec<TraceId> {
        todo!()
        //     let mut traces = Vec::new();
        //     for stmt in stmts {
        //         match stmt.variant {
        //             HirEagerStmt::ConditionFlow { ref branches } => {
        //                 for (branch_idx, branch) in branches.iter().enumerate() {
        //                     traces.push(self.new_proc_branch_trace(
        //                         parent_id,
        //                         indent,
        //                         stmt.clone(),
        //                         branch.clone(),
        //                         branch_idx.try_into().unwrap(),
        //                         history.clone(),
        //                     ))
        //                 }
        //             }
        //             _ => traces.push(self.new_proc_stmt_trace(
        //                 parent_id,
        //                 indent,
        //                 stmt.clone(),
        //                 history.clone(),
        //             )),
        //         }
        //     }
        //     traces
    }
}
