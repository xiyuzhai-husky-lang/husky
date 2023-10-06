use crate::{
    view::{TraceViewData, TraceViewTokenData},
    *,
};
use husky_token_protocol::TokenClass;
#[cfg(feature = "mock")]
use husky_visual_protocol::mock::MockVisualProtocol;
use husky_visual_protocol::{IsVisualProtocol, VisualComponent};

/// synced across server and client
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceCache<VisualProtocol: IsVisualProtocol> {
    root_trace_ids: TraceIdRange,
    entries: Vec<TraceCacheEntry>,
    visual_components: Vec<VisualComponent<VisualProtocol>>,
}

/// methods
impl<VisualProtocol: IsVisualProtocol> TraceCache<VisualProtocol> {
    pub fn root_trace_ids(&self) -> TraceIdRange {
        self.root_trace_ids
    }

    pub(crate) fn take_actions(&mut self, actions: Vec<TraceAction>) {
        todo!()
    }

    pub(crate) fn take_action(&mut self, action: TraceAction) {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceCacheEntry {
    view_data: TraceViewData,
    /// None means not calculated
    subtraces: Option<TraceIdRange>,
}

impl TraceCacheEntry {
    pub fn view_data(&self) -> &TraceViewData {
        &self.view_data
    }

    pub fn subtraces(&self) -> Option<TraceIdRange> {
        self.subtraces
    }
}

impl<VisualProtocol: IsVisualProtocol> std::ops::Index<TraceIdRange>
    for TraceCache<VisualProtocol>
{
    type Output = [TraceCacheEntry];

    fn index(&self, trace_id_range: TraceIdRange) -> &Self::Output {
        &self.entries[trace_id_range.start().index()..trace_id_range.end().index()]
    }
}

#[cfg(feature = "mock")]
impl TraceCache<MockVisualProtocol> {
    pub fn new_mock() -> Self {
        use TokenClass::*;
        Self {
            root_trace_ids: TraceIdRange::new_mock(0, 1),
            entries: vec![TraceCacheEntry {
                view_data: TraceViewData::new_mock([
                    ("let", OtherKeyword),
                    ("a", Variable),
                    ("=", Punctuation),
                    ("x", Parameter),
                ]),
                subtraces: None,
            }],
            visual_components: vec![],
        }
    }
}
