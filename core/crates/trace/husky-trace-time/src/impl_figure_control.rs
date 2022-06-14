use feature_gen::FeatureStmtVariant;
use semantics_eager::ProcStmtVariant;
use serde::{Deserialize, Serialize};
use trace::*;
use vm::{HistoryEntry, LoopFrameData, MutationData};

use super::*;

impl HuskyTraceTime {
    pub fn figure_control(&mut self, trace: &Trace, focus: &Focus) -> FigureControlData {
        let key = FigureControlKey::new(&trace.props, focus);
        if let Some(control) = self.figure_controls.get(&key) {
            control.clone()
        } else {
            let control = self.gen_figure_control(trace);
            self.figure_controls.insert(key, control.clone());
            control
        }
    }
    pub fn gen_figure_control(&mut self, trace: &Trace) -> FigureControlData {
        match trace.variant {
            TraceVariant::Main(_)
            | TraceVariant::FeatureStmt(_)
            | TraceVariant::FeatureBranch(_)
            | TraceVariant::FeatureExpr(_)
            | TraceVariant::FeatureCallInput { .. }
            | TraceVariant::FuncStmt { .. }
            | TraceVariant::EagerExpr { .. }
            | TraceVariant::CallHead { .. } => FigureControlData::default(),
            TraceVariant::ProcStmt { ref stmt, .. } => match stmt.variant {
                ProcStmtVariant::Loop { .. } => FigureControlData::loop_default(&trace.props),
                _ => FigureControlData::default(),
            },
            TraceVariant::LoopFrame { .. } => FigureControlData::loop_default(
                &self.trace(trace.props.opt_parent_id.unwrap()).props,
            ),
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
        focus: &Focus,
        new_control: FigureControlData,
    ) {
        self.figure_controls.insert(
            FigureControlKey::new(&self.trace(trace_id).props, focus),
            new_control,
        );
    }
}
