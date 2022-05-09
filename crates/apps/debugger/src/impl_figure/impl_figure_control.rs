use feature::FeatureStmtVariant;
use trace::TraceVariant;
use vm::{HistoryEntry, LoopFrameData, MutationData};

use super::*;

#[derive(Debug, Default, Serialize, Clone)]
#[serde(tag = "kind")]
pub struct FigureControlProps {
    opt_mutation_selection: Option<u8>,
}

impl FigureControlProps {
    pub fn loop_frame_default(loop_trace: &Trace) -> Self {
        match loop_trace.variant {
            TraceVariant::ProcStmt {
                ref stmt,
                ref history,
            } => match history.get(stmt).unwrap() {
                HistoryEntry::Loop { mutations, .. } => Self::mutations_default(mutations),
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    pub fn mutations_default(mutations: &[MutationData]) -> Self {
        let opt_mutation_selection = if mutations.len() > 0 { Some(0) } else { None };
        FigureControlProps {
            opt_mutation_selection,
        }
    }
}

impl Debugger {
    pub fn figure_control(&self, id: TraceId, focus: &Focus) -> FigureControlProps {
        let trace = self.trace(id);
        match trace.variant {
            TraceVariant::Main(_)
            | TraceVariant::FeatureStmt(_)
            | TraceVariant::FeatureBranch(_)
            | TraceVariant::FeatureExpr(_)
            | TraceVariant::FeatureCallInput { .. }
            | TraceVariant::FuncStmt { .. }
            | TraceVariant::ProcStmt { .. }
            | TraceVariant::EagerExpr { .. }
            | TraceVariant::CallHead { .. } => FigureControlProps::default(),
            TraceVariant::LoopFrame {
                ref loop_stmt,
                ref body_instruction_sheet,
                ref body_stmts,
                ref loop_frame_data,
            } => FigureControlProps::loop_frame_default(&self.trace(trace.parent.unwrap())),
            TraceVariant::ProcBranch {
                ref stmt,
                branch_idx,
                ref history,
                ..
            } => match history.get(stmt).unwrap() {
                HistoryEntry::BranchGroup {
                    branch_entered,
                    mutations,
                    ..
                } => {
                    if branch_idx == *branch_entered {
                        FigureControlProps::mutations_default(mutations)
                    } else {
                        FigureControlProps::default()
                    }
                }
                _ => panic!(),
            },
        }
    }

    pub fn update_figure_control(&self, id: TraceId, new_control: FigureControlProps) {
        todo!()
    }
}
