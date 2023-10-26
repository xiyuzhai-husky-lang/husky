use crate::{
    view::{TraceViewData, TraceViewTokenData},
    *,
};
use husky_token_protocol::TokenClass;
#[cfg(feature = "mock")]
use husky_visual_protocol::mock::MockVisualProtocol;
use husky_visual_protocol::{IsVisualComponent, IsVisualProtocol};

/// synced across server and client
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceCache<VisualComponent> {
    /// None means not set
    root_trace_ids: Option<TraceIdRange>,
    entries: Vec<TraceCacheEntry>,
    visual_components: Vec<VisualComponent>,
}

impl<VisualComponent: IsVisualComponent> Default for TraceCache<VisualComponent> {
    fn default() -> Self {
        Self {
            root_trace_ids: Default::default(),
            entries: Default::default(),
            visual_components: Default::default(),
        }
    }
}

/// methods
impl<VisualComponent: IsVisualComponent> TraceCache<VisualComponent> {
    pub fn root_trace_ids(&self) -> Option<TraceIdRange> {
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

impl<VisualComponent: IsVisualComponent> std::ops::Index<TraceIdRange>
    for TraceCache<VisualComponent>
{
    type Output = [TraceCacheEntry];

    fn index(&self, trace_id_range: TraceIdRange) -> &Self::Output {
        &self.entries[trace_id_range.start().index()..trace_id_range.end().index()]
    }
}

#[cfg(feature = "mock")]
impl TraceCache<()> {
    pub fn new_mock() -> Self {
        use TokenClass::*;
        Self {
            root_trace_ids: Some(TraceIdRange::new_mock(0, 1)),
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
