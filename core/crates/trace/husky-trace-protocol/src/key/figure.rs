use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub enum FigureCanvasKey {
    Null,
    Feature {
        trace_id: TraceId,
        attention: Attention,
        enters: Vec<TraceId>,
        arrivals: Vec<TraceId>,
        pins: Vec<TraceId>,
    },
    Eager {
        trace_id: TraceId,
        pins: Vec<TraceId>,
    },
}

impl FigureCanvasKey {
    pub fn from_trace_data(
        trace_data: &TraceData,
        attention: &Attention,
        enters: Vec<TraceId>,
        arrivals: Vec<TraceId>,
        pins: Vec<TraceId>,
    ) -> FigureCanvasKey {
        Self::new(
            trace_data.kind,
            trace_data.id,
            attention,
            enters,
            arrivals,
            pins,
        )
    }

    pub fn new(
        trace_kind: TraceKind,
        trace_id: TraceId,
        attention: &Attention,
        enters: Vec<TraceId>,
        arrivals: Vec<TraceId>,
        pins: Vec<TraceId>,
    ) -> FigureCanvasKey {
        match trace_kind {
            TraceKind::Main
            | TraceKind::FeatureStmt
            | TraceKind::FeatureBranch
            | TraceKind::FeatureExpr
            | TraceKind::FeatureCallArgument => FigureCanvasKey::Feature {
                trace_id,
                attention: attention.clone(),
                enters,
                arrivals,
                pins,
            },
            TraceKind::FuncStmt
            | TraceKind::ProcStmt
            | TraceKind::FuncBranch
            | TraceKind::ProcBranch
            | TraceKind::EagerExpr
            | TraceKind::LoopFrame => FigureCanvasKey::Eager { trace_id, pins },
            TraceKind::CallHead | TraceKind::EagerCallArgument => FigureCanvasKey::Null,
        }
    }
}
