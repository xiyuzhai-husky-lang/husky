use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceData {
    pub opt_parent_id: Option<TraceId>,
    pub id: TraceId,
    pub kind: TraceKind,
    pub indent: Indent,
    pub lines: Vec<TraceLineData>,
    pub can_have_subtraces: bool,
    pub reachable: bool,
    pub opt_arrival_indicator: Option<FeatureId>,
}

impl TraceData {
    pub fn associated_trace_ids(&self) -> Vec<TraceId> {
        let mut associated_trace_ids = vec![];
        for line in &self.lines {
            for token in &line.tokens {
                if token.value == "]" || token.value == ")" || token.value == "}" {
                    continue;
                }
                if let Some(associated_trace_id) = token.opt_associated_trace_id {
                    associated_trace_ids.push(associated_trace_id)
                }
            }
        }
        associated_trace_ids
    }

    pub fn has_subtraces(&self, has_sample_id: bool) -> bool {
        match self.kind {
            TraceKind::Main
            | TraceKind::EntityFeatureLazy
            | TraceKind::FeatureExprLazy
            | TraceKind::Module
            | TraceKind::FeatureBranch
            | TraceKind::LoopFrame => true,
            TraceKind::CallHead
            | TraceKind::FeatureCallArgument
            | TraceKind::EagerCallArgument
            | TraceKind::FeatureStmt => false,
            TraceKind::FuncStmt
            | TraceKind::EagerExpr
            | TraceKind::EagerStmt
            | TraceKind::FuncBranch
            | TraceKind::EagerBranch => self.can_have_subtraces,
            TraceKind::EntityFeatureEager | TraceKind::FeatureExprEager => {
                has_sample_id && self.can_have_subtraces
            }
        }
    }
}
