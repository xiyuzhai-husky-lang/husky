use super::*;

impl Devtime {
    pub(crate) fn loop_subtraces(
        &mut self,
        parent: &Trace,
        loop_kind: VMLoopKind,
        loop_stmt: &SynStmtIdx,
        body_stmts: &Arc<Vec<SynStmtIdx>>,
        stack_snapshot: &StackSnapshot,
        body_instruction_sheet: &Instructions,
    ) -> Vec<TraceId> {
        let sample_id = self.state.presentation().opt_sample_id().unwrap();
        let evaluator = self.runtime().evaluator(sample_id);
        let frames = exec_loop_debug(
            &self.runtime() as &DevRuntime,
            unsafe { evaluator.some_ctx() },
            loop_kind,
            &body_instruction_sheet,
            stack_snapshot,
            self.vm_config(),
        );
        frames
            .into_iter()
            .map(|loop_frame_data| {
                self.new_trace(
                    Some(parent.id()),
                    parent.raw_data.indent + 2,
                    TraceVariant::LoopFrame {
                        loop_stmt: loop_stmt.clone(),
                        body_stmts: body_stmts.clone(),
                        body_instruction_sheet: body_instruction_sheet.clone(),
                        loop_frame_data,
                    },
                )
            })
            .collect()
    }

    pub(crate) fn loop_frame_subtraces(
        &mut self,
        loop_stmt: &SynStmtIdx,
        stmts: &[SynStmtIdx],
        instruction_sheet: &Instructions,
        loop_frame_data: &LoopFrameData,
        parent: &Trace,
    ) -> Vec<TraceId> {
        let sample_id = self.state.presentation().opt_sample_id().unwrap();
        let evaluator = self.runtime().evaluator(sample_id);
        let history = exec_debug(
            self.runtime(),
            unsafe { evaluator.some_ctx() },
            instruction_sheet,
            (&loop_frame_data.stack_snapshot).into(),
            self.vm_config(),
        );
        let mut subtraces: Vec<_> =
            self.proc_stmts_traces(parent.id(), parent.raw_data.indent + 2, stmts, &history);
        match loop_stmt.variant {
            HirEagerStmt::Loop {
                ref loop_variant, ..
            } => match loop_variant {
                LoopVariant::For { .. } | LoopVariant::ForExt { .. } => (),
                LoopVariant::While { condition } => subtraces.insert(
                    0,
                    self.new_eager_expr_trace(
                        condition.clone(),
                        history.clone(),
                        Some(parent),
                        parent.raw_data.indent + 2,
                    ),
                ),
                LoopVariant::DoWhile { condition } => subtraces.push(self.new_eager_expr_trace(
                    condition.clone(),
                    history.clone(),
                    Some(parent),
                    parent.raw_data.indent + 2,
                )),
            },
            _ => panic!(),
        }
        subtraces
    }

    pub(crate) fn proc_branch_subtraces(
        &mut self,
        stmts: &[SynStmtIdx],
        instruction_sheet: &Instructions,
        stack_snapshot: &StackSnapshot,
        parent: &Trace,
    ) -> Vec<TraceId> {
        let sample_id = self.state.presentation().opt_sample_id().unwrap();
        let evaluator = self.runtime().evaluator(sample_id);
        let history = exec_debug(
            self.runtime().upcast(),
            unsafe { evaluator.some_ctx() },
            instruction_sheet,
            stack_snapshot.into(),
            self.vm_config(),
        );
        self.proc_stmts_traces(parent.id(), parent.raw_data.indent + 4, stmts, &history)
    }
}
