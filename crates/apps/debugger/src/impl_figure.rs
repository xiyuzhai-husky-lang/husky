mod figure_control;
mod figure_focus;

pub use figure_control::*;
pub use figure_focus::*;

use serde::Serialize;
use trace::TraceVariant;

use crate::*;

impl Debugger {
    pub async fn figure(&self, id: TraceId, focus: &Focus) -> FigureProps {
        self.runtime.lock().unwrap().figure(id, focus)
    }

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
