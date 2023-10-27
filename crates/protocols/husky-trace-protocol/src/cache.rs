mod action;
mod entry;

pub use self::action::TraceCacheAction;

use self::entry::*;
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
    actions: Vec<TraceCacheAction<VisualComponent>>,
}

/// methods
impl<VisualComponent: IsVisualComponent> TraceCache<VisualComponent> {
    pub fn new(root_traces: impl Iterator<Item = (TraceId, TraceViewData)>) -> Self {
        let mut root_trace_ids: Vec<TraceId> = vec![];
        let mut entries: Vec<TraceCacheEntry> = vec![];
        for (root_trace_id, view_data) in root_traces {
            debug_assert_eq!(root_trace_ids.len(), root_trace_id.index());
            root_trace_ids.push(root_trace_id);
            entries.push(TraceCacheEntry::new(view_data))
        }
        Self {
            root_trace_ids,
            entries,
            visual_components: vec![],
            actions: vec![],
        }
    }

    pub fn root_trace_ids(&self) -> &[TraceId] {
        self.root_trace_ids.as_ref()
    }
}

impl<VisualComponent: IsVisualComponent> std::ops::Index<TraceId> for TraceCache<VisualComponent> {
    type Output = TraceCacheEntry;

    fn index(&self, id: TraceId) -> &Self::Output {
        &self.entries[id.index()]
    }
}
