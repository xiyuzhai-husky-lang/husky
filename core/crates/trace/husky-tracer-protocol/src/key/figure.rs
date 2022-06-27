use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub enum FigureCanvasKey {
    Null,
    Feature {
        trace_id: TraceId,
        attention: Attention,
    },
    Eager {
        trace_id: TraceId,
    },
}

impl FigureCanvasKey {
    pub fn from_trace_data(trace_data: &TraceData, attention: &Attention) -> FigureCanvasKey {
        Self::new(trace_data.kind, trace_data.id, attention)
    }

    pub fn new(trace_kind: TraceKind, trace_id: TraceId, attention: &Attention) -> FigureCanvasKey {
        match trace_kind {
            TraceKind::Main
            | TraceKind::FeatureStmt
            | TraceKind::FeatureBranch
            | TraceKind::FeatureExpr
            | TraceKind::FeatureCallArgument => FigureCanvasKey::Feature {
                trace_id,
                attention: attention.clone(),
            },
            TraceKind::FuncStmt
            | TraceKind::ProcStmt
            | TraceKind::ProcBranch
            | TraceKind::EagerExpr
            | TraceKind::LoopFrame => FigureCanvasKey::Eager { trace_id },
            TraceKind::CallHead | TraceKind::EagerCallArgument => FigureCanvasKey::Null,
        }
    }
}
