use feature::FeatureStmtVariant;
use serde::{Deserialize, Serialize};
use trace::TraceVariant;
use vm::{HistoryEntry, LoopFrameData, MutationData};

use super::*;

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(tag = "kind")]
pub struct FigureControlProps {
    opt_mutation_selection: Option<u8>,
}

impl FigureControlProps {
    pub fn loop_default(loop_trace: &Trace) -> Self {
        let control_props = match loop_trace.variant {
            TraceVariant::ProcStmt {
                ref stmt,
                ref history,
            } => match history.get(stmt).unwrap() {
                HistoryEntry::Loop { mutations, .. } => Self::mutations_default(mutations),
                _ => panic!(),
            },
            _ => panic!(),
        };
        control_props
    }

    pub fn mutations_default(mutations: &[MutationData]) -> Self {
        let opt_mutation_selection = if mutations.len() > 0 { Some(0) } else { None };
        FigureControlProps {
            opt_mutation_selection,
        }
    }
}

impl HuskyRuntime {
    pub fn figure_control(&mut self, trace: &Trace, focus: &Focus) -> FigureControlProps {
        let key = focus.figure_control_key(trace);
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
                ProcStmtVariant::Loop { .. } => FigureControlProps::loop_default(trace),
                _ => FigureControlProps::default(),
            },
            TraceVariant::LoopFrame { .. } => {
                FigureControlProps::loop_default(&self.trace(trace.parent.unwrap()))
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
                        FigureControlProps::mutations_default(mutations)
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
        self.figure_controls
            .insert(focus.figure_control_key(&self.trace(trace_id)), new_control);
    }
}
