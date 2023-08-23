use husky_print_utils::msg_once;
use husky_trace::*;
use husky_vm::HistoryEntry;

use super::*;

impl Devtime {
    #[inline(always)]
    pub fn figure_control(&mut self, trace_id: TraceId) -> FigureControlData {
        let trace = self.trace(trace_id);
        let key = FigureControlKey::new(
            trace.raw_data.opt_parent_id,
            trace.raw_data.kind,
            trace.raw_data.id,
            self.state.presentation(),
        );
        if let Some(control) = self.state.figure_controls.get(&key) {
            control.clone()
        } else {
            let control = self.gen_figure_control_data(trace_id);
            self.state.figure_controls.insert_new(key, control.clone());
            control
        }
    }

    #[inline(always)]
    pub fn gen_figure_control_data(&mut self, trace_id: TraceId) -> FigureControlData {
        todo!()
        // let trace = self.trace(trace_id);
        // match trace.variant {
        //     TraceVariant::Main(..)
        //     | TraceVariant::EntityVal { .. }
        //     | TraceVariant::Module { .. }
        //     | TraceVariant::ValStmt(_)
        //     | TraceVariant::ValBranch(_)
        //     | TraceVariant::LazyExpr(_)
        //     | TraceVariant::ValCallArgument { .. }
        //     | TraceVariant::FuncStmt { .. }
        //     | TraceVariant::EagerExpr { .. }
        //     | TraceVariant::CallHead { .. }
        //     | TraceVariant::EagerCallArgument { .. } => FigureControlData::default(),
        //     TraceVariant::EagerStmt {
        //         ref stmt,
        //         ref history,
        //         ..
        //     } => {
        //         todo!()
        //         //     match stmt.variant {
        //         //     HirEagerStmt::Loop { .. } => match history.get(stmt)? {
        //         //         HistoryEntry::Loop { mutations, .. } => {
        //         //             FigureControlData::mutations_default(mutations.len())
        //         //         }
        //         //         _ => {
        //         //             p!(stmt.file, stmt.range);
        //         //             panic!()
        //         //         }
        //         //     },
        //         //     _ => FigureControlData::default(),
        //         // }
        //     }
        //     TraceVariant::LoopFrame { .. } => {
        //         self.gen_figure_control_data(trace.raw_data.opt_parent_id.unwrap())
        //     }
        //     TraceVariant::FuncBranch {
        //         ref stmt,
        //         branch_idx,
        //         ref history,
        //         ..
        //     } => match history.get(stmt)? {
        //         HistoryEntry::ControlFlow {
        //             opt_branch_entered, ..
        //         } => {
        //             if Some(branch_idx) == *opt_branch_entered {
        //                 msg_once!("todo");
        //                 FigureControlData::default()
        //             } else {
        //                 FigureControlData::default()
        //             }
        //         }
        //         _ => panic!(),
        //     },
        //     TraceVariant::EagerBranch {
        //         ref stmt,
        //         branch_idx,
        //         ref history,
        //         ..
        //     } => match history.get(stmt)? {
        //         HistoryEntry::ControlFlow {
        //             opt_branch_entered: branch_entered,
        //             mutations,
        //             ..
        //         } => {
        //             if Some(branch_idx) == *branch_entered {
        //                 FigureControlData::mutations_default(mutations.len())
        //             } else {
        //                 FigureControlData::default()
        //             }
        //         }
        //         _ => panic!(),
        //     },
        // }
    }

    pub(crate) fn update_figure_controls(&mut self) -> __VMResult<()> {
        todo!()
        // if let Some(active_trace_id) = self.opt_active_trace_id() {
        //     self.update_figure_control(active_trace_id)?;
        // }
        // for pin in self.state.presentation().pins().to_vec().into_iter() {
        //     self.update_figure_control(pin)?;
        // }
        // Ok(())
    }

    pub(crate) fn update_figure_control(&mut self, trace_id: TraceId) {
        let key = self.gen_figure_control_key(trace_id);
        if !self.state.figure_controls.contains(&key) {
            let figure_control_data = self.gen_figure_control_data(trace_id);
            self.state
                .figure_controls
                .insert_new(key, figure_control_data.clone());
        }
    }

    pub fn set_figure_control(
        &mut self,
        _trace_id: TraceId,
        _new_figure_control_data: FigureControlData,
    ) {
        todo!()
        // let key = self.gen_figure_control_key(trace_id);
        // DevtimeStageChangeM::Ok(
        //     self.state
        //         .figure_controls
        //         .insert_new(key, new_figure_control_data)?,
        // )
    }

    fn gen_figure_control_key(&self, trace_id: TraceId) -> FigureControlKey {
        let trace_raw_data = &self.trace(trace_id).raw_data;
        FigureControlKey::new(
            trace_raw_data.opt_parent_id,
            trace_raw_data.kind,
            trace_raw_data.id,
            self.state.presentation(),
        )
    }
}
