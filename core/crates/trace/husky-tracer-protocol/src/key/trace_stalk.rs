use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TraceStalkKey {
    Null,
    Eager {
        trace_id: TraceId,
    },
    Lazy {
        sample_id: SampleId,
        trace_id: TraceId,
    },
}

impl TraceStalkKey {
    pub fn from_trace_data(sample_id: SampleId, trace_data: &TraceData) -> Self {
        Self::new(sample_id, trace_data.kind, trace_data.id)
    }
    pub fn from_trace_raw_data(sample_id: SampleId, trace_raw_data: &TraceRawData) -> Self {
        Self::new(sample_id, trace_raw_data.kind, trace_raw_data.id)
    }

    pub fn new(sample_id: SampleId, trace_kind: TraceKind, trace_id: TraceId) -> Self {
        match trace_kind {
            TraceKind::Main
            | TraceKind::FeatureStmt
            | TraceKind::FeatureBranch
            | TraceKind::FeatureExpr
            | TraceKind::FeatureCallInput => TraceStalkKey::Lazy {
                trace_id,
                sample_id,
            },
            TraceKind::FuncStmt
            | TraceKind::ProcStmt
            | TraceKind::ProcBranch
            | TraceKind::LoopFrame
            | TraceKind::EagerExpr => TraceStalkKey::Eager { trace_id },
            TraceKind::CallHead => TraceStalkKey::Null,
        }
    }
}
