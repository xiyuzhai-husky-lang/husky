use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub enum FigureCanvasKey {
    Null,
    Feature { this: TraceId, focus: Focus },
    Eager { this: TraceId },
}

impl FigureCanvasKey {
    pub fn new(trace_data: &TraceData, focus: &Focus) -> FigureCanvasKey {
        match trace_data.kind {
            TraceKind::Main
            | TraceKind::FeatureStmt
            | TraceKind::FeatureBranch
            | TraceKind::FeatureExpr
            | TraceKind::FeatureCallInput => FigureCanvasKey::Feature {
                this: trace_data.id,
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
