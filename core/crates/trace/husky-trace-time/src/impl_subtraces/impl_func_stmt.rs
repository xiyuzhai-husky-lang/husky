use super::*;

impl HuskyTraceTime {
    pub(crate) fn func_branch_subtraces(
        &mut self,
        stmts: &[Arc<FuncStmt>],
        instruction_sheet: &InstructionSheet,
        stack_snapshot: &StackSnapshot<'static>,
        parent: &Trace,
    ) -> Vec<TraceId> {
        let history = exec_debug(
            self.eval_time().upcast(),
            instruction_sheet,
            stack_snapshot,
            self.vm_config(),
        );
        self.func_stmts_traces(parent.id(), parent.raw_data.indent + 4, stmts, &history)
    }
}
