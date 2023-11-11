use crate::*;

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TraceStalkKey {
    Null,
    Lazy {
        sample_id: SampleId,
        trace_id: TraceId,
    },
}

impl std::fmt::Debug for TraceStalkKey {
    fn fmt(&self, f: &mut __private::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Null => write!(f, "Null"),
            Self::Lazy {
                sample_id,
                trace_id,
            } => f.write_fmt(format_args!(
                "Lazy {{ sample_id: {}, trace_id: {} }}",
                sample_id.0, trace_id
            )),
        }
    }
}

impl TraceStalkKey {
    pub fn from_trace_data(sample_id: SampleId, trace_data: &TraceData) -> Self {
        Self::new(sample_id, trace_data.kind, trace_data.id)
    }

    pub fn new(sample_id: SampleId, trace_kind: TraceKind, trace_id: TraceId) -> Self {
        match trace_kind {
            TraceKind::Main
            | TraceKind::EntityFeatureLazy
            | TraceKind::EntityFeatureEager
            | TraceKind::FeatureStmt
            | TraceKind::FeatureBranch
            | TraceKind::FeatureExprLazy
            | TraceKind::FeatureExprEager
            | TraceKind::FeatureCallArgument => TraceStalkKey::Lazy {
                trace_id,
                sample_id,
            },
            TraceKind::Module
            | TraceKind::LoopFrame
            | TraceKind::FuncStmt
            | TraceKind::EagerStmt
            | TraceKind::EagerBranch
            | TraceKind::FuncBranch
            | TraceKind::CallHead
            | TraceKind::EagerExpr
            | TraceKind::EagerCallArgument => TraceStalkKey::Null,
        }
    }
}
