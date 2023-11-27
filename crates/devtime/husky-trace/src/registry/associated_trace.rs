use crate::{db::TraceDb, TraceId};
use husky_token_info::TokenInfoSource;

pub(crate) trait IsAssociatedTraceRegistry: Sized {
    fn get_or_issue_associated_trace(
        &mut self,
        source: TokenInfoSource,
        db: &::salsa::Db,
    ) -> Option<TraceId>;

    fn get_or_issue_associated_trace_id(
        &mut self,
        source: TokenInfoSource,
        db: &::salsa::Db,
    ) -> Option<TraceId> {
        self.get_or_issue_associated_trace(source, db)
            .map(Into::into)
    }
}

pub(crate) struct VoidAssociatedTraceRegistry;

impl IsAssociatedTraceRegistry for VoidAssociatedTraceRegistry {
    fn get_or_issue_associated_trace(
        &mut self,
        _source: TokenInfoSource,
        _db: &::salsa::Db,
    ) -> Option<TraceId> {
        None
    }
}
