use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SubtracesKey {
    Trivial {
        trace_id: TraceId,
    },
    Stalkwise {
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
            | TraceKind::Module
            | TraceKind::FeatureStmt
            | TraceKind::FeatureBranch
            | TraceKind::FuncStmt
            | TraceKind::ProcStmt
            | TraceKind::FuncBranch
            | TraceKind::ProcBranch
            | TraceKind::LoopFrame
            | TraceKind::EagerExpr
            | TraceKind::EntityFeatureLazy
            | TraceKind::FeatureExprLazy => SubtracesKey::Trivial { trace_id },
            TraceKind::FeatureCallArgument | TraceKind::CallHead | TraceKind::EagerCallArgument => {
                SubtracesKey::Null
            }
            TraceKind::EntityFeatureEager | TraceKind::FeatureExprEager => match opt_sample_id {
                Some(sample_id) => SubtracesKey::Stalkwise {
                    trace_id,
                    sample_id,
                },
                None => SubtracesKey::Null,
            },
        }
    }
}
