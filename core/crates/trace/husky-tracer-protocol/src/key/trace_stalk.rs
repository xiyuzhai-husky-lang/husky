use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TraceStalkKey {
    Null,
    Eager {
        trace_id: TraceId,
    },
    Lazy {
        sample_idx: SampleIdx,
        trace_id: TraceId,
    },
}

impl TraceStalkKey {
    pub fn from_trace_data(sample_idx: SampleIdx, trace_data: &TraceData) -> Self {
        Self::new(sample_idx, trace_data.kind, trace_data.id)
    }
    pub fn from_trace_raw_data(sample_idx: SampleIdx, trace_raw_data: &TraceRawData) -> Self {
        Self::new(sample_idx, trace_raw_data.kind, trace_raw_data.id)
    }

    pub fn new(sample_idx: SampleIdx, trace_kind: TraceKind, trace_id: TraceId) -> Self {
        match trace_kind {
            TraceKind::Main
            | TraceKind::FeatureStmt
            | TraceKind::FeatureBranch
            | TraceKind::FeatureExpr
            | TraceKind::FeatureCallInput => TraceStalkKey::Lazy {
                trace_id,
                sample_idx,
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
