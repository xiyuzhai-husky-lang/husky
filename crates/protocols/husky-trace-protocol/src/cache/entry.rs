use super::*;
use rustc_hash::FxHashMap;
use vec_like::SmallVecSet;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceCacheEntry<TraceProtocol: IsTraceProtocol> {
    view_data: TraceViewData,
    /// None means not calculated
    subtrace_ids: Option<Vec<TraceId>>,
    associated_trace_ids: SmallVecSet<TraceId, 2>,
    expanded: bool,
    stalks: FxHashMap<TraceProtocol::Pedestal, TraceStalk>,
}

impl<TraceProtocol: IsTraceProtocol> TraceCacheEntry<TraceProtocol> {
    pub fn new(view_data: TraceViewData) -> Self {
        Self {
            view_data,
            subtrace_ids: None,
            associated_trace_ids: Default::default(),
            expanded: false,
            stalks: todo!(),
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
}
