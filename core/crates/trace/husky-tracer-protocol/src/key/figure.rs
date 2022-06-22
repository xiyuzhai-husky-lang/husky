use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub enum FigureCanvasKey {
    Null,
    Feature {
        trace_id: TraceId,
        attention: Attention,
    },
    Eager {
        this: TraceId,
    },
}

impl FigureCanvasKey {
    pub fn from_trace_raw_data(
        trace_raw_data: &TraceData,
        attention: &Attention,
    ) -> FigureCanvasKey {
        Self::new(trace_raw_data.kind, trace_raw_data.id, attention)
    }

    pub fn new(trace_kind: TraceKind, trace_id: TraceId, attention: &Attention) -> FigureCanvasKey {
        match trace_kind {
            TraceKind::Main
            | TraceKind::FeatureStmt
            | TraceKind::FeatureBranch
            | TraceKind::FeatureExpr
            | TraceKind::FeatureCallInput => FigureCanvasKey::Feature {
                trace_id,
                attention: attention.clone(),
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
