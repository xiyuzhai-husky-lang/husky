pub(crate) mod action;
mod entry;

pub use self::action::TraceCenterAction;
pub use self::entry::TraceCenterEntry;

use crate::{view::TraceViewData, *};
use vec_like::VecPairMap;

/// synced across server and client
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceCenter<TraceProtocol: IsTraceProtocol> {
    pedestal: TraceProtocol::Pedestal,
    /// None means not set
    root_trace_ids: Vec<TraceId>,
    entries: VecPairMap<TraceId, TraceCenterEntry<TraceProtocol>>,
    visual_components: Vec<<TraceProtocol::VisualProtocol as IsVisualProtocol>::VisualComponent>,
    actions: Vec<TraceCenterAction<TraceProtocol>>,
}

/// methods
impl<TraceProtocol: IsTraceProtocol> TraceCenter<TraceProtocol> {
    pub fn new(root_traces: impl Iterator<Item = (TraceId, TraceViewData)>) -> Self {
        let mut root_trace_ids: Vec<TraceId> = vec![];
        let mut entries: VecPairMap<TraceId, TraceCenterEntry<TraceProtocol>> = Default::default();
        for (root_trace_id, view_data) in root_traces {
            root_trace_ids.push(root_trace_id);
            entries
                .insert_new((root_trace_id, TraceCenterEntry::new(view_data)))
                .unwrap()
        }
        Self {
            pedestal: Default::default(),
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
    ) -> smallvec::SmallVec<[TraceCenterAction<TraceProtocol>; 3]> {
        assert!(previous_cache_actions_len < self.actions.len());
        self.actions[previous_cache_actions_len..]
            .iter()
            .map(|action| action.clone())
            .collect()
    }

    pub(crate) fn is_trace_cached(&self, trace_id: TraceId) -> bool {
        self.entries.has(trace_id)
    }

    pub(crate) fn trace_listing(&self) -> Vec<TraceId> {
        let mut trace_listings: Vec<TraceId> = vec![];
        for &root_trace_id in &self.root_trace_ids {
            self.trace_listing_aux(root_trace_id, &mut trace_listings)
        }
        trace_listings
    }

    fn trace_listing_aux(&self, trace_id: TraceId, trace_listings: &mut Vec<TraceId>) {
        trace_listings.push(trace_id);
        let entry = &self[trace_id];
        // todo: only list those shown
        for &associated_trace_id in entry.associated_trace_ids() {
            self.trace_listing_aux(associated_trace_id, trace_listings)
        }
        if entry.expanded() {
            for &subtrace_id in entry.subtrace_ids().unwrap() {
                self.trace_listing_aux(subtrace_id, trace_listings)
            }
        }
    }

    pub fn pedestal(&self) -> <TraceProtocol as IsTraceProtocol>::Pedestal {
        self.pedestal
    }
}

impl<TraceProtocol: IsTraceProtocol> std::ops::Index<TraceId> for TraceCenter<TraceProtocol> {
    type Output = TraceCenterEntry<TraceProtocol>;

    fn index(&self, id: TraceId) -> &Self::Output {
        &self.entries[id].1
    }
}

impl<TraceProtocol: IsTraceProtocol> std::ops::IndexMut<TraceId> for TraceCenter<TraceProtocol> {
    #[track_caller]
    fn index_mut(&mut self, id: TraceId) -> &mut Self::Output {
        &mut self.entries.get_mut(id).unwrap().1
    }
}
