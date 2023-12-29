pub(crate) mod action;
mod entry;

pub use self::action::TraceCacheAction;
pub use self::entry::TraceCacheEntry;

use crate::{view::TraceViewData, *};
use vec_like::VecPairMap;

/// synced across server and client
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceCache<TraceProtocol: IsTraceProtocol> {
    /// None means not set
    root_trace_ids: Vec<TraceId>,
    entries: VecPairMap<TraceId, TraceCacheEntry<TraceProtocol>>,
    visual_components: Vec<<TraceProtocol::VisualProtocol as IsVisualProtocol>::VisualComponent>,
    actions: Vec<TraceCacheAction<TraceProtocol>>,
}

/// methods
impl<TraceProtocol: IsTraceProtocol> TraceCache<TraceProtocol> {
    pub fn new(root_traces: impl Iterator<Item = (TraceId, TraceViewData)>) -> Self {
        let mut root_trace_ids: Vec<TraceId> = vec![];
        let mut entries: VecPairMap<TraceId, TraceCacheEntry<TraceProtocol>> = Default::default();
        for (root_trace_id, view_data) in root_traces {
            root_trace_ids.push(root_trace_id);
            entries
                .insert_new((root_trace_id, TraceCacheEntry::new(view_data)))
                .unwrap()
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

    pub(crate) fn actions_len(&self) -> usize {
        self.actions.len()
    }

    pub(crate) fn reproduce_cache_actions(
        &self,
        previous_cache_actions_len: usize,
    ) -> smallvec::SmallVec<[TraceCacheAction<TraceProtocol>; 3]> {
        assert!(previous_cache_actions_len < self.actions.len());
        self.actions[previous_cache_actions_len..]
            .iter()
            .map(|action| action.clone())
            .collect()
    }

    pub(crate) fn is_trace_cached(&self, trace_id: TraceId) -> bool {
        self.entries.has(trace_id)
    }
}

impl<TraceProtocol: IsTraceProtocol> std::ops::Index<TraceId> for TraceCache<TraceProtocol> {
    type Output = TraceCacheEntry<TraceProtocol>;

    fn index(&self, id: TraceId) -> &Self::Output {
        &self.entries[id].1
    }
}

impl<TraceProtocol: IsTraceProtocol> std::ops::IndexMut<TraceId> for TraceCache<TraceProtocol> {
    #[track_caller]
    fn index_mut(&mut self, id: TraceId) -> &mut Self::Output {
        &mut self.entries.get_mut(id).unwrap().1
    }
}
