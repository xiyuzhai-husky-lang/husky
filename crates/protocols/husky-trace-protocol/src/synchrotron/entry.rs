use super::*;
use rustc_hash::FxHashMap;
use serde_with::serde_as;
use vec_like::{OrderedSmallVecSet, SmallVecSet};

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceSynchrotronEntry<TraceProtocol: IsTraceProtocol> {
    view_data: TraceViewData,
    /// None means not calculated
    subtrace_ids: Option<Vec<TraceId>>,
    associated_trace_ids_shown: SmallVecSet<TraceId, 2>,
    expanded: bool,
    #[serde_as(as = "Vec<(_, _)>")]
    stalks: FxHashMap<TraceProtocol::Pedestal, TraceStalk>,
    figures: FxHashMap<(TraceProtocol::Pedestal, AccompanyingTraceIds), TraceProtocol::Figure>,
}

impl<TraceProtocol: IsTraceProtocol> TraceSynchrotronEntry<TraceProtocol> {
    pub fn new(view_data: TraceViewData) -> Self {
        Self {
            view_data,
            subtrace_ids: None,
            associated_trace_ids_shown: Default::default(),
            expanded: false,
            stalks: Default::default(),
            figures: Default::default(),
        }
    }

    pub fn view_data(&self) -> &TraceViewData {
        &self.view_data
    }

    pub fn subtrace_ids(&self) -> Option<&[TraceId]> {
        self.subtrace_ids.as_ref().map(|ids| ids.as_ref())
    }

    pub fn expanded(&self) -> bool {
        self.expanded
    }

    pub(crate) fn has_stalk(&self, pedestal: TraceProtocol::Pedestal) -> bool {
        self.stalks.contains_key(&pedestal)
    }

    /// written in this way to avoid cloning of AccompanyingTraceIds
    pub(crate) fn has_figure(
        &self,
        pedestal: TraceProtocol::Pedestal,
        accompanying_trace_ids: AccompanyingTraceIds,
    ) -> (bool, AccompanyingTraceIds) {
        let key = (pedestal, accompanying_trace_ids);
        (self.figures.contains_key(&key), key.1)
    }

    pub fn stalk(&self, pedestal: TraceProtocol::Pedestal) -> &TraceStalk {
        &self.stalks[&pedestal]
    }

    pub(super) fn toggle_expansion(&mut self) {
        self.expanded = !self.expanded
    }

    pub(super) fn toggle_associated_traces(&mut self, associated_trace_id: TraceId) {
        self.associated_trace_ids_shown.toggle(associated_trace_id)
    }

    pub fn associated_trace_ids(&self) -> &[TraceId] {
        self.associated_trace_ids_shown.as_ref()
    }

    pub(crate) fn cache_subtraces(&mut self, subtrace_ids: Vec<TraceId>) {
        assert!(self.subtrace_ids.is_none());
        self.subtrace_ids = Some(subtrace_ids)
    }

    pub(super) fn cache_stalk(&mut self, pedestal: TraceProtocol::Pedestal, stalk: TraceStalk) {
        // self.stalks.get_value_mut_or_insert_with(pedestal, f);
        debug_assert!(self.stalks.insert(pedestal, stalk).is_none());
    }

    pub(crate) fn cache_figure(
        &mut self,
        pedestal: TraceProtocol::Pedestal,
        accompanying_trace_ids: AccompanyingTraceIds,
        figure: TraceProtocol::Figure,
    ) {
        assert!(self
            .figures
            .insert((pedestal, accompanying_trace_ids), figure)
            .is_none())
    }
}
