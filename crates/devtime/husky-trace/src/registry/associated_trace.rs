use crate::Trace;
use husky_token_info::TokenInfoSource;
use husky_trace_protocol::id::TraceId;

pub(crate) trait IsAssociatedTraceRegistry: Default {
    fn get_or_issue_associated_trace(&mut self, source: TokenInfoSource) -> Option<Trace>;
    fn get_or_issue_associated_trace_id(&mut self, source: TokenInfoSource) -> Option<TraceId> {
        self.get_or_issue_associated_trace(source).map(Into::into)
    }
}

#[derive(Default)]
pub(crate) struct VoidAssociatedTraceRegistry {}

impl IsAssociatedTraceRegistry for VoidAssociatedTraceRegistry {
    fn get_or_issue_associated_trace(&mut self, source: TokenInfoSource) -> Option<Trace> {
        None
    }
}
