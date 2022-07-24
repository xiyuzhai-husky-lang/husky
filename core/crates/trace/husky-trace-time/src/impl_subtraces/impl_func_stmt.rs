use super::*;

impl HuskyTraceTime {
    pub(crate) fn func_branch_subtraces(
        &mut self,
        stmts: &[Arc<FuncStmt>],
        instruction_sheet: &InstructionSheet,
        stack_snapshot: &StackSnapshot<'static>,
        parent: &Trace,
    ) -> Vec<TraceId> {
        let sample_id = self.restriction.opt_sample_id().unwrap();
        let evaluator = self.eval_time().evaluator(sample_id);
        let history = exec_debug(
            self.eval_time(),
            unsafe { evaluator.some_ctx() },
            instruction_sheet,
            stack_snapshot.into(),
            self.vm_config(),
        );
        self.func_stmts_traces(parent.id(), parent.raw_data.indent + 4, stmts, &history)
    }
}
