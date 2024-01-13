pub(crate) mod action;
mod entry;

pub use self::action::TraceSynchrotronAction;
pub use self::entry::TraceSynchrotronEntry;

use crate::{view::TraceViewData, *};
use husky_value_protocol::presentation::{
    ValuePresentationSynchrotron, ValuePresentationSynchrotronStatus,
};
use vec_like::VecPairMap;

/// contains information about traces that are synced across server and client
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceSynchrotron<TraceProtocol: IsTraceProtocol> {
    pedestal: TraceProtocol::Pedestal,
    /// None means not set
    root_trace_ids: Vec<TraceId>,
    entries: VecPairMap<TraceId, TraceSynchrotronEntry<TraceProtocol>>,
    visual_components: Vec<<TraceProtocol::VisualProtocol as IsVisualProtocol>::VisualComponent>,
    actions: Vec<TraceSynchrotronAction<TraceProtocol>>,
    value_presentation_synchrotron: ValuePresentationSynchrotron,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceSynchrotronStatus {
    actions_len: usize,
    value_presentation_synchrotron_status: ValuePresentationSynchrotronStatus,
}

/// methods
impl<TraceProtocol: IsTraceProtocol> TraceSynchrotron<TraceProtocol> {
    pub fn new(root_traces: impl Iterator<Item = (TraceId, TraceViewData)>) -> Self {
        let mut root_trace_ids: Vec<TraceId> = vec![];
        let mut entries: VecPairMap<TraceId, TraceSynchrotronEntry<TraceProtocol>> =
            Default::default();
        for (root_trace_id, view_data) in root_traces {
            root_trace_ids.push(root_trace_id);
            entries
                .insert_new((root_trace_id, TraceSynchrotronEntry::new(view_data)))
                .unwrap()
        }
        Self {
            value_presentation_synchrotron: Default::default(),
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

    pub(crate) fn status(&self) -> TraceSynchrotronStatus {
        TraceSynchrotronStatus {
            actions_len: self.actions.len(),
            value_presentation_synchrotron_status: self.value_presentation_synchrotron.status(),
        }
    }

    pub(crate) fn diff_actions(
        &self,
        previous_trace_synchrotron_status: TraceSynchrotronStatus,
    ) -> smallvec::SmallVec<[TraceSynchrotronAction<TraceProtocol>; 3]> {
        assert!(previous_trace_synchrotron_status.actions_len < self.actions.len());
        self.actions[previous_trace_synchrotron_status.actions_len..]
            .iter()
            .map(|action| action.clone())
            .chain(
                self.value_presentation_synchrotron
                    .diff_actions()
                    .iter()
                    .map(|action| action.clone().into()),
            )
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

    pub(crate) fn value_presentation_synchrotron_mut(
        &mut self,
    ) -> &mut husky_value_protocol::presentation::ValuePresentationSynchrotron {
        &mut self.value_presentation_synchrotron
    }
}

impl<TraceProtocol: IsTraceProtocol> std::ops::Index<TraceId> for TraceSynchrotron<TraceProtocol> {
    type Output = TraceSynchrotronEntry<TraceProtocol>;

    fn index(&self, id: TraceId) -> &Self::Output {
        &self.entries[id].1
    }
}

impl<TraceProtocol: IsTraceProtocol> std::ops::IndexMut<TraceId>
    for TraceSynchrotron<TraceProtocol>
{
    #[track_caller]
    fn index_mut(&mut self, id: TraceId) -> &mut Self::Output {
        &mut self.entries.get_mut(id).unwrap().1
    }
}
