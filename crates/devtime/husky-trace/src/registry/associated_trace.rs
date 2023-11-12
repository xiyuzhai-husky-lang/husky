use crate::{db::TraceDb, Trace};
use husky_token_info::TokenInfoSource;
use husky_trace_protocol::id::TraceId;

pub(crate) trait IsAssociatedTraceRegistry: Sized {
    fn get_or_issue_associated_trace(
        &mut self,
        source: TokenInfoSource,
        db: &dyn TraceDb,
    ) -> Option<Trace>;

    fn get_or_issue_associated_trace_id(
        &mut self,
        source: TokenInfoSource,
        db: &dyn TraceDb,
    ) -> Option<TraceId> {
        self.get_or_issue_associated_trace(source, db)
            .map(Into::into)
    }
}

pub(crate) struct VoidAssociatedTraceRegistry;

impl IsAssociatedTraceRegistry for VoidAssociatedTraceRegistry {
    fn get_or_issue_associated_trace(
        &mut self,
        source: TokenInfoSource,
        db: &dyn TraceDb,
    ) -> Option<Trace> {
        None
    }
}
