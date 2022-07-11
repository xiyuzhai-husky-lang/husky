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
    pub fn new(
        trace_kind: TraceKind,
        trace_id: TraceId,
        opt_sample_id: Option<SampleId>,
    ) -> SubtracesKey {
        match trace_kind {
            TraceKind::Main
            | TraceKind::FeatureStmt
            | TraceKind::FeatureBranch
            | TraceKind::FuncStmt
            | TraceKind::ProcStmt
            | TraceKind::FuncBranch
            | TraceKind::ProcBranch
            | TraceKind::LoopFrame
            | TraceKind::EagerExpr => SubtracesKey::Simple { trace_id },
            TraceKind::FeatureCallArgument | TraceKind::CallHead | TraceKind::EagerCallArgument => {
                SubtracesKey::Null
            }
            TraceKind::FeatureExpr => match opt_sample_id {
                Some(sample_id) => SubtracesKey::FeatureExprStalk {
                    trace_id,
                    sample_id,
                },
                None => SubtracesKey::Null,
            },
        }
    }
}
