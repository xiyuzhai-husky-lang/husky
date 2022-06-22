use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SubtracesKey {
    Simple {
        trace_id: TraceId,
    },
    FeatureExprStalk {
        trace_id: TraceId,
        sample_idx: SampleIdx,
    },
    Null,
}

impl SubtracesKey {
    pub fn new(attention: &Attention, trace_kind: TraceKind, trace_id: TraceId) -> SubtracesKey {
        match trace_kind {
            TraceKind::Main
            | TraceKind::FeatureStmt
            | TraceKind::FeatureBranch
            | TraceKind::FuncStmt
            | TraceKind::ProcStmt
            | TraceKind::ProcBranch
            | TraceKind::LoopFrame
            | TraceKind::EagerExpr => SubtracesKey::Simple { trace_id },
            TraceKind::FeatureCallInput | TraceKind::CallHead => SubtracesKey::Null,
            TraceKind::FeatureExpr => match attention {
                Attention::Specific {
                    sample_idx: sample_idx,
                } => SubtracesKey::FeatureExprStalk {
                    trace_id,
                    sample_idx: *sample_idx,
                },
                Attention::Generic { .. } => SubtracesKey::Null,
            },
        }
    }
}
