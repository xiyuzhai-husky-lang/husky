use super::*;

impl Devtime {
    pub(crate) fn func_branch_subtraces(
        &mut self,
        stmts: &[SynStmtIdx],
        instruction_sheet: &Instructions,
        stack_snapshot: &StackSnapshot,
        parent: &Trace,
    ) -> Vec<TraceId> {
        let sample_id = self.state.presentation().opt_sample_id().unwrap();
        let evaluator = self.runtime().evaluator(sample_id);
        let history = exec_debug(
            self.runtime(),
            unsafe { evaluator.some_ctx() },
            instruction_sheet,
            stack_snapshot.into(),
            self.vm_config(),
        );
        self.func_stmts_traces(parent.id(), parent.raw_data.indent + 4, stmts, &history)
    }
}
