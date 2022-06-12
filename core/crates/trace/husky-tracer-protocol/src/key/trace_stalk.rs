use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TraceStalkKey {
    Null,
    Eager { trace_id: TraceId },
    Lazy { trace_id: TraceId, input_id: usize },
}

impl TraceStalkKey {
    pub fn new(trace: &TraceData, input_id: usize) -> Self {
        match trace.kind {
            TraceKind::Main
            | TraceKind::FeatureStmt
            | TraceKind::FeatureBranch
            | TraceKind::FeatureExpr
            | TraceKind::FeatureCallInput => TraceStalkKey::Lazy {
                trace_id: trace.id,
                input_id,
            },
            TraceKind::FuncStmt
            | TraceKind::ProcStmt
            | TraceKind::ProcBranch
            | TraceKind::LoopFrame
            | TraceKind::EagerExpr => TraceStalkKey::Eager { trace_id: trace.id },
            TraceKind::CallHead => TraceStalkKey::Null,
        }
    }
}
