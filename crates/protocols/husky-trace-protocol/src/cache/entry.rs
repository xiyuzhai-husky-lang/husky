use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceCacheEntry {
    view_data: TraceViewData,
    /// None means not calculated
    subtraces: Option<Vec<TraceId>>,
    /// None means not calculated
    associated_traces: Option<Vec<TraceId>>,
    expanded: bool,
}

impl TraceCacheEntry {
    pub fn new(view_data: TraceViewData) -> Self {
        Self {
            view_data,
            subtraces: None,
            associated_traces: None,
            expanded: false,
        }
    }

    pub fn view_data(&self) -> &TraceViewData {
        &self.view_data
    }

    pub fn subtraces(&self) -> Option<&[TraceId]> {
        self.subtraces.as_ref().map(|ids| ids.as_ref())
    }

    pub fn expanded(&self) -> bool {
        self.expanded
    }

    pub(super) fn toggle_expansion(&mut self) {
        self.expanded = !self.expanded
    }
}
