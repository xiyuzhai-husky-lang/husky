use trace::TraceVariant;
use vm::LoopFrameData;

use super::*;

#[derive(Debug, Default, Serialize, Clone)]
#[serde(tag = "kind")]
pub struct FigureControl {
    opt_mutation_selection: Option<u8>,
}

impl FigureControl {
    pub fn loop_frame_default<'eval>(loop_frame_data: &LoopFrameData<'eval>) -> Self {
        FigureControl {
            opt_mutation_selection: if loop_frame_data.mutations.len() > 0 {
                Some(0)
            } else {
                None
            },
        }
    }
}

impl Debugger {
    pub fn figure_control(&self, id: TraceId, focus: &Focus) -> FigureControl {
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
            | TraceVariant::CallHead { .. } => FigureControl::default(),
            TraceVariant::LoopFrame {
                ref loop_stmt,
                ref body_instruction_sheet,
                ref body_stmts,
                ref loop_frame_data,
            } => FigureControl::loop_frame_default(loop_frame_data),
        }
    }

    pub fn update_figure_control(&self, id: TraceId, new_control: FigureControl) {
        todo!()
    }
}
