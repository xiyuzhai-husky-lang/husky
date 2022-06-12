use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SubtracesKey {
    Simple { trace_id: TraceId },
    FeatureExprStalk { trace_id: TraceId, input_id: usize },
    Null,
}

impl SubtracesKey {
    pub fn new(focus: &Focus, trace_kind: TraceKind, trace_id: TraceId) -> SubtracesKey {
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
            TraceKind::FeatureExpr => match focus.opt_input_id {
                Some(input_id) => SubtracesKey::FeatureExprStalk { trace_id, input_id },
                None => SubtracesKey::Null,
            },
        }
    }
}
