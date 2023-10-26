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
    root_trace_ids: Vec<TraceId>,
    entries: Vec<TraceCacheEntry>,
    visual_components: Vec<VisualComponent>,
}

/// methods
impl<VisualComponent: IsVisualComponent> TraceCache<VisualComponent> {
    pub fn new(root_traces: impl Iterator<Item = (TraceId, TraceViewData)>) -> Self {
        let mut root_trace_ids: Vec<TraceId> = vec![];
        let mut entries: Vec<TraceCacheEntry> = vec![];
        for (root_trace_id, view_data) in root_traces {
            root_trace_ids.push(root_trace_id);
            entries.push(TraceCacheEntry {
                view_data,
                subtraces: None,
                associated_traces: None,
            })
        }
        Self {
            root_trace_ids,
            entries,
            visual_components: vec![],
        }
    }

    pub fn root_trace_ids(&self) -> &[TraceId] {
        self.root_trace_ids.as_ref()
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
    subtraces: Option<Vec<TraceId>>,
    /// None means not calculated
    associated_traces: Option<Vec<TraceId>>,
}

impl TraceCacheEntry {
    pub fn view_data(&self) -> &TraceViewData {
        &self.view_data
    }

    pub fn subtraces(&self) -> Option<&[TraceId]> {
        self.subtraces.as_ref().map(|ids| ids.as_ref())
    }
}

#[cfg(feature = "mock")]
impl TraceCache<()> {
    pub fn new_mock() -> Self {
        use TokenClass::*;
        Self {
            root_trace_ids: TraceId::new_mocks([0, 1]),
            entries: vec![TraceCacheEntry {
                view_data: TraceViewData::new_mock([
                    ("let", OtherKeyword),
                    ("a", Variable),
                    ("=", Punctuation),
                    ("x", Parameter),
                ]),
                subtraces: None,
                associated_traces: None,
            }],
            visual_components: vec![],
        }
    }
}

impl<VisualComponent: IsVisualComponent> std::ops::Index<TraceId> for TraceCache<VisualComponent> {
    type Output = TraceCacheEntry;

    fn index(&self, id: TraceId) -> &Self::Output {
        &self.entries[id.index()]
    }
}
