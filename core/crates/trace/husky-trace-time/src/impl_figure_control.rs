use feature_gen::FeatureStmtVariant;
use semantics_eager::ProcStmtVariant;
use serde::{Deserialize, Serialize};
use trace::*;
use vm::{HistoryEntry, LoopFrameData, MutationData};

use super::*;

impl HuskyTraceTime {
    #[inline(always)]
    pub fn figure_control(
        &mut self,
        trace_id: TraceId,
        attention: &Attention,
    ) -> FigureControlData {
        let trace = self.trace(trace_id);
        let key = FigureControlKey::new(
            trace.raw_data.opt_parent_id,
            trace.raw_data.kind,
            trace.raw_data.id,
            attention,
        );
        if let Some(control) = self.figure_controls.get(&key) {
            control.clone()
        } else {
            let control = self.gen_figure_control(trace_id);
            self.figure_controls.insert(key, control.clone());
            control
        }
    }

    #[inline(always)]
    pub fn gen_figure_control(&mut self, trace_id: TraceId) -> FigureControlData {
        let trace = self.trace(trace_id);
        match trace.variant {
            TraceVariant::Main(_)
            | TraceVariant::FeatureStmt(_)
            | TraceVariant::FeatureBranch(_)
            | TraceVariant::FeatureExpr(_)
            | TraceVariant::FeatureCallArgument { .. }
            | TraceVariant::FuncStmt { .. }
            | TraceVariant::EagerExpr { .. }
            | TraceVariant::CallHead { .. } => FigureControlData::default(),
            TraceVariant::ProcStmt {
                ref stmt,
                ref history,
            } => match stmt.variant {
                ProcStmtVariant::Loop { .. } => match history.get(stmt) {
                    Some(HistoryEntry::Loop { mutations, .. }) => {
                        FigureControlData::mutations_default(mutations.len())
                    }
                    None => Default::default(),
                    _ => {
                        p!(stmt.file, stmt.range);
                        panic!()
                    }
                },
                _ => FigureControlData::default(),
            },
            TraceVariant::LoopFrame { .. } => {
                self.gen_figure_control(trace.raw_data.opt_parent_id.unwrap())
            }
            TraceVariant::ProcBranch {
                ref stmt,
                branch_idx,
                ref history,
                ..
            } => match history.get(stmt).unwrap() {
                HistoryEntry::ControlFlow {
                    opt_branch_entered: branch_entered,
                    mutations,
                    ..
                } => {
                    if Some(branch_idx) == *branch_entered {
                        todo!()
                        // FigureControlData::mutations_default(mutations)
                    } else {
                        FigureControlData::default()
                    }
                }
                _ => panic!(),
            },
        }
    }

    pub fn update_figure_control(
        &mut self,
        trace_id: TraceId,
        attention: &Attention,
        new_control: FigureControlData,
    ) {
        let trace_raw_data = &self.trace(trace_id).raw_data;
        self.figure_controls.insert(
            FigureControlKey::new(
                trace_raw_data.opt_parent_id,
                trace_raw_data.kind,
                trace_raw_data.id,
                attention,
            ),
            new_control,
        );
    }
}
