

use super::*;
use rustc_hash::FxHashMap;
use serde_with::serde_as;
use vec_like::SmallVecSet;

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceCenterEntry<TraceProtocol: IsTraceProtocol> {
    view_data: TraceViewData,
    /// None means not calculated
    subtrace_ids: Option<Vec<TraceId>>,
    associated_trace_ids: SmallVecSet<TraceId, 2>,
    expanded: bool,
    #[serde_as(as = "Vec<(_, _)>")]
    stalks: FxHashMap<TraceProtocol::Pedestal, TraceStalk>,
}

impl<TraceProtocol: IsTraceProtocol> TraceCenterEntry<TraceProtocol> {
    pub fn new(view_data: TraceViewData) -> Self {
        Self {
            view_data,
            subtrace_ids: None,
            associated_trace_ids: Default::default(),
            expanded: false,
            stalks: Default::default(),
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

    pub(super) fn toggle_expansion(&mut self) {
        self.expanded = !self.expanded
    }

    pub(super) fn toggle_associated_traces(&mut self, associated_trace_id: TraceId) {
        self.associated_trace_ids.toggle(associated_trace_id)
    }

    pub(crate) fn set_subtraces(&mut self, subtrace_ids: Vec<TraceId>) {
        assert!(self.subtrace_ids.is_none());
        self.subtrace_ids = Some(subtrace_ids)
    }

    pub fn associated_trace_ids(&self) -> &[TraceId] {
        self.associated_trace_ids.as_ref()
    }

    pub(crate) fn cache_stalk(
        &mut self,
        pedestal: <TraceProtocol as IsTraceProtocol>::Pedestal,
        f: impl FnOnce() -> TraceStalk,
    ) {
        // self.stalks.get_value_mut_or_insert_with(pedestal, f);
        self.stalks.entry(pedestal).or_insert_with(f);
    }
}
