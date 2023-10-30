use vec_like::SmallVecSet;

use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceCacheEntry {
    view_data: TraceViewData,
    /// None means not calculated
    subtrace_ids: Option<Vec<TraceId>>,
    associated_traces: SmallVecSet<TraceId, 2>,
    expanded: bool,
}

impl TraceCacheEntry {
    pub fn new(view_data: TraceViewData) -> Self {
        Self {
            view_data,
            subtrace_ids: None,
            associated_traces: Default::default(),
            expanded: false,
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

    pub(super) fn toggle_associated_traces(
        &mut self,
        associated_trace_id: TraceId,
        is_active: bool, // for verification purpose
    ) {
        debug_assert_eq!(self.associated_traces.has(associated_trace_id), is_active);
        self.associated_traces.toggle(associated_trace_id)
    }

    pub(crate) fn set_subtraces(&mut self, subtrace_ids: Vec<TraceId>) {
        assert!(self.subtrace_ids.is_none());
        self.subtrace_ids = Some(subtrace_ids)
    }
}
