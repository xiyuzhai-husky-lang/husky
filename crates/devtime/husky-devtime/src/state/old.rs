use crate::*;

#[must_use]
pub struct HuskyDevtimeOldState {}

impl HuskyDevtimeOldState {
    pub fn new(presentation: Presentation, trace_nodes: Vec<TraceNode>) -> Self {
        Self {}
    }

    pub fn try_match(&self, trace_node: &TraceNode) -> Option<&TraceNode> {
        todo!()
    }
}
