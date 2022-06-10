use feature::FeatureStmtVariant;
use semantics_eager::ProcStmtVariant;
use serde::{Deserialize, Serialize};
use trace::*;
use vm::{HistoryEntry, LoopFrameData, MutationData};

use super::*;

impl HuskyDebugTime {
    pub fn figure_control(&mut self, trace: &Trace, focus: &Focus) -> FigureControlProps {
        let key = FigureControlKey::new(&trace.props);
        if let Some(control) = self.figure_controls.get(&key) {
            control.clone()
        } else {
            let control = self.gen_figure_control(trace);
            self.figure_controls.insert(key, control.clone());
            control
        }
    }
    pub fn gen_figure_control(&mut self, trace: &Trace) -> FigureControlProps {
        match trace.variant {
            TraceVariant::Main(_)
            | TraceVariant::FeatureStmt(_)
            | TraceVariant::FeatureBranch(_)
            | TraceVariant::FeatureExpr(_)
            | TraceVariant::FeatureCallInput { .. }
            | TraceVariant::FuncStmt { .. }
            | TraceVariant::EagerExpr { .. }
            | TraceVariant::CallHead { .. } => FigureControlProps::default(),
            TraceVariant::ProcStmt { ref stmt, .. } => match stmt.variant {
                ProcStmtVariant::Loop { .. } => FigureControlProps::loop_default(&trace.props),
                _ => FigureControlProps::default(),
            },
            TraceVariant::LoopFrame { .. } => {
                FigureControlProps::loop_default(&self.trace(trace.props.opt_parent.unwrap()).props)
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
                        // FigureControlProps::mutations_default(mutations)
                    } else {
                        FigureControlProps::default()
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
        new_control: FigureControlProps,
    ) {
        self.figure_controls.insert(
            FigureControlKey::new(&self.trace(trace_id).props),
            new_control,
        );
    }
}
