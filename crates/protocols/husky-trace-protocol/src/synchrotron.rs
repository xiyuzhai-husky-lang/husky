pub(crate) mod action;
mod entry;

pub use self::action::TraceSynchrotronAction;
use self::action::TraceSynchrotronActionsDiff;
pub use self::entry::TraceSynchrotronEntry;

use crate::{view::TraceViewData, *};
use husky_value_protocol::presentation::synchrotron::{
    ValuePresentationSynchrotron, ValuePresentationSynchrotronStatus,
};
use husky_visual_protocol::synchrotron::{VisualSynchrotron, VisualSynchrotronStatus};
use vec_like::VecPairMap;

/// contains information about traces that are synced across server and client
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceSynchrotron<TraceProtocol: IsTraceProtocol> {
    pedestal: TraceProtocol::Pedestal,
    accompanying_trace_ids: AccompanyingTraceIds,
    root_trace_ids: Vec<TraceId>,
    focused_trace_id: Option<TraceId>,
    entries: VecPairMap<TraceId, TraceSynchrotronEntry<TraceProtocol>>,
    actions: Vec<TraceSynchrotronAction<TraceProtocol>>,
    // child synchrotrons
    value_presentation_synchrotron: ValuePresentationSynchrotron,
    visual_synchrotron: VisualSynchrotron,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceSynchrotronStatus {
    actions_len: usize,
    value_presentation_synchrotron_status: ValuePresentationSynchrotronStatus,
    visual_synchrotron_status: VisualSynchrotronStatus,
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
            pedestal: Default::default(),
            root_trace_ids,
            entries,
            actions: vec![],
            value_presentation_synchrotron: Default::default(),
            visual_synchrotron: Default::default(),
            focused_trace_id: None,
            accompanying_trace_ids: Default::default(),
        }
    }

    pub fn root_trace_ids(&self) -> &[TraceId] {
        self.root_trace_ids.as_ref()
    }

    pub(crate) fn status(&self) -> TraceSynchrotronStatus {
        TraceSynchrotronStatus {
            actions_len: self.actions.len(),
            value_presentation_synchrotron_status: self.value_presentation_synchrotron.status(),
            visual_synchrotron_status: self.visual_synchrotron.status(),
        }
    }

    pub(crate) fn actions_diff(
        &self,
        previous_trace_synchrotron_status: TraceSynchrotronStatus,
    ) -> TraceSynchrotronActionsDiff<TraceProtocol> {
        assert!(previous_trace_synchrotron_status.actions_len < self.actions.len());
        let actions = self.actions[previous_trace_synchrotron_status.actions_len..]
            .iter()
            .map(|action| action.clone())
            .collect();
        let value_presentation_actions_diff = self
            .value_presentation_synchrotron
            .actions_diff(previous_trace_synchrotron_status.value_presentation_synchrotron_status);
        let visual_actions_diff = self
            .visual_synchrotron
            .actions_diff(previous_trace_synchrotron_status.visual_synchrotron_status);
        TraceSynchrotronActionsDiff::new(
            actions,
            value_presentation_actions_diff,
            visual_actions_diff,
        )
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
        for &associated_trace_id in entry.associated_trace_ids() {
            self.trace_listing_aux(associated_trace_id, trace_listings)
        }
        if entry.expanded() {
            for &subtrace_id in entry.subtrace_ids().unwrap() {
                self.trace_listing_aux(subtrace_id, trace_listings)
            }
        }
    }

    pub fn pedestal(&self) -> TraceProtocol::Pedestal {
        self.pedestal
    }

    pub(crate) fn value_presentation_synchrotron_mut(
        &mut self,
    ) -> &mut ValuePresentationSynchrotron {
        &mut self.value_presentation_synchrotron
    }

    pub fn followed_trace_id(&self) -> Option<TraceId> {
        self.focused_trace_id
    }

    pub(crate) fn accompanying_trace_ids(&self) -> &AccompanyingTraceIds {
        &self.accompanying_trace_ids
    }

    pub(crate) fn visual_synchrotron_mut(&mut self) -> &mut VisualSynchrotron {
        &mut self.visual_synchrotron
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
