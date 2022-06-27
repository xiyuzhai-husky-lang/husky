use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SubtracesKey {
    Simple {
        trace_id: TraceId,
    },
    FeatureExprStalk {
        trace_id: TraceId,
        sample_id: SampleId,
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
            TraceKind::FeatureCallArgument | TraceKind::CallHead | TraceKind::EagerCallArgument => {
                SubtracesKey::Null
            }
            TraceKind::FeatureExpr => match attention {
                Attention::Specific {
                    sample_id: sample_id,
                } => SubtracesKey::FeatureExprStalk {
                    trace_id,
                    sample_id: *sample_id,
                },
                Attention::Generic { .. } => SubtracesKey::Null,
            },
        }
    }
}
