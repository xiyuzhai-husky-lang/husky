use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceCacheEntry {
    view_data: TraceViewData,
    /// None means not calculated
    subtrace_ids: Option<Vec<TraceId>>,
    /// None means not calculated
    associated_traces: Option<Vec<TraceId>>,
    expanded: bool,
}

impl TraceCacheEntry {
    pub fn new(view_data: TraceViewData) -> Self {
        Self {
            view_data,
            subtrace_ids: None,
            associated_traces: None,
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

    pub(crate) fn set_subtraces(&mut self, subtrace_ids: Vec<TraceId>) {
        assert!(self.subtrace_ids.is_none());
        self.subtrace_ids = Some(subtrace_ids)
    }
}
