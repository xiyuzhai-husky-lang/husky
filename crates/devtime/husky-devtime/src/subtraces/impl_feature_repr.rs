use crate::*;
use husky_eval::Evaluator;

impl Devtime {
    pub fn val_repr_subtraces(
        &mut self,
        parent: &Trace,
        feature_repr: &ValRepr,
    ) -> Option<Vec<TraceId>> {
        todo!()
        // match feature_repr {
        //     ValRepr::Value { .. } => todo!(),
        //     ValRepr::LazyExpr(_) => todo!(),
        //     ValRepr::LazyBody(block) => Some(self.val_lazy_block_subtraces(parent, block)),
        //     ValRepr::FuncBody(block) => self.val_func_block_subtraces(parent, block),
        //     ValRepr::ProcBody(block) => self.val_proc_block_subtraces(parent, block),
        //     ValRepr::TargetInput { .. } => None,
        // }
    }

    pub fn val_lazy_block_subtraces(
        &mut self,
        parent: &Trace,
        feature_block: &ValBlock,
    ) -> Vec<TraceId> {
        todo!()
        // feature_block
        //     .stmts
        //     .iter()
        //     .map(|stmt| self.feature_stmt_traces(parent, stmt.clone()))
        //     .flatten()
        //     .collect()
    }

    // pub fn val_func_block_subtraces(
    //     &mut self,
    //     parent: &Trace,
    //     feature_block: &FeatureFuncBody,
    // ) -> Option<Vec<TraceId>> {
    //     let instruction_sheet: &Instructions = &feature_block.instruction_sheet;
    //     let mut arguments = vec![];
    //     let sample_id = self.state.presentation().opt_sample_id()?;
    //     if let Some(ref this_repr) = feature_block.opt_this {
    //         arguments.push(
    //             self.runtime()
    //                 .eval_feature_repr(this_repr, sample_id)
    //                 .unwrap(),
    //         )
    //     }
    //     let sample_id = self.state.presentation().opt_sample_id().unwrap();
    //     let eval_time = self.runtime();
    //     let evaluator: Evaluator<'_> = eval_time.evaluator(sample_id);
    //     let stack = arguments.into_iter().into();
    //     let history = exec_debug(
    //         self.runtime(),
    //         Some(&evaluator),
    //         instruction_sheet,
    //         stack,
    //         self.vm_config(),
    //     );
    //     let mut subtraces = vec![];
    //     subtraces.extend(self.func_stmts_traces(parent.id(), 4, &feature_block.stmts, &history));
    //     Some(subtraces)
    // }

    // pub fn val_proc_block_subtraces(
    //     &mut self,
    //     parent: &Trace,
    //     feature_block: &FeatureProcBody,
    // ) -> Option<Vec<TraceId>> {
    //     let instruction_sheet: &Instructions = &feature_block.instruction_sheet;
    //     let mut arguments = vec![];
    //     let sample_id = self.state.presentation().opt_sample_id()?;
    //     if let Some(ref this_repr) = feature_block.opt_this {
    //         arguments.push(
    //             self.runtime()
    //                 .eval_feature_repr(this_repr, sample_id)
    //                 .unwrap(),
    //         )
    //     }
    //     let sample_id = self.state.presentation().opt_sample_id().unwrap();
    //     let eval_time = self.runtime();
    //     let evaluator: Evaluator<'_> = eval_time.evaluator(sample_id);
    //     let stack = arguments.into_iter().into();
    //     let history = exec_debug(
    //         self.runtime(),
    //         Some(&evaluator),
    //         instruction_sheet,
    //         stack,
    //         self.vm_config(),
    //     );
    //     let mut subtraces = vec![];
    //     subtraces.extend(self.proc_stmts_traces(parent.id(), 4, &feature_block.stmts, &history));
    //     Some(subtraces)
    // }
}
