use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TraceStalkKey {
    Null,
    Lazy {
        sample_id: SampleId,
        trace_id: TraceId,
    },
}

impl TraceStalkKey {
    pub fn from_trace_data(sample_id: SampleId, trace_data: &TraceData) -> Self {
        Self::new(sample_id, trace_data.kind, trace_data.id)
    }

    pub fn new(sample_id: SampleId, trace_kind: TraceKind, trace_id: TraceId) -> Self {
        match trace_kind {
            TraceKind::Main
            | TraceKind::FeatureStmt
            | TraceKind::FeatureBranch
            | TraceKind::FeatureExpr
            | TraceKind::FeatureCallArgument => TraceStalkKey::Lazy {
                trace_id,
                sample_id,
            },
            TraceKind::Module
            | TraceKind::LoopFrame
            | TraceKind::FuncStmt
            | TraceKind::ProcStmt
            | TraceKind::ProcBranch
            | TraceKind::FuncBranch
            | TraceKind::CallHead
            | TraceKind::EagerExpr
            | TraceKind::EagerCallArgument => TraceStalkKey::Null,
        }
    }
}
