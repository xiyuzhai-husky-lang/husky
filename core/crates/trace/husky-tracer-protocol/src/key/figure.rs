use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub enum FigureCanvasKey {
    Null,
    Feature { trace_id: TraceId, focus: Focus },
    Eager { this: TraceId },
}

impl FigureCanvasKey {
    pub fn from_trace_raw_data(trace_raw_data: &TraceRawData, focus: &Focus) -> FigureCanvasKey {
        Self::new(trace_raw_data.kind, trace_raw_data.id, focus)
    }

    pub fn new(trace_kind: TraceKind, trace_id: TraceId, focus: &Focus) -> FigureCanvasKey {
        match trace_kind {
            TraceKind::Main
            | TraceKind::FeatureStmt
            | TraceKind::FeatureBranch
            | TraceKind::FeatureExpr
            | TraceKind::FeatureCallInput => FigureCanvasKey::Feature {
                trace_id,
                focus: focus.clone(),
            },
            TraceKind::FuncStmt => todo!(),
            TraceKind::ProcStmt => todo!(),
            TraceKind::ProcBranch => todo!(),
            TraceKind::LoopFrame => todo!(),
            TraceKind::EagerExpr => todo!(),
            TraceKind::CallHead => todo!(),
        }
    }
}
