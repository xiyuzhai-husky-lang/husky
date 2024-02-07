use crate::Trace;
use husky_token_info::TokenInfoSource;

pub(crate) trait IsAssocTraceRegistry: Sized {
    fn get_or_issue_associated_trace(
        &mut self,
        source: TokenInfoSource,
        db: &::salsa::Db,
    ) -> Option<Trace>;

    fn get_or_issue_associated_trace_id(
        &mut self,
        source: TokenInfoSource,
        db: &::salsa::Db,
    ) -> Option<Trace> {
        self.get_or_issue_associated_trace(source, db)
            .map(Into::into)
    }
}

pub(crate) struct VoidAssocTraceRegistry;

impl IsAssocTraceRegistry for VoidAssocTraceRegistry {
    fn get_or_issue_associated_trace(
        &mut self,
        _source: TokenInfoSource,
        _db: &::salsa::Db,
    ) -> Option<Trace> {
        None
    }
}