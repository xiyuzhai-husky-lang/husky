use super::*;
use husky_item_path_interface::ItemPathIdInterface;
use serde_with::serde_as;
use smallvec::SmallVec;
use vec_like::SmallVecSet;

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceSynchrotronEntry<TraceProtocol: IsTraceProtocol> {
    var_deps: SmallVec<[ItemPathIdInterface; 2]>,
    view_data: TraceViewData,
    /// None means not calculated
    subtrace_ids: Option<Vec<TraceId>>,
    assoc_trace_ids_shown: SmallVecSet<TraceId, 2>,
    expanded: bool,
    #[serde_as(as = "Vec<(_, _)>")]
    stalks: FxHashMap<TraceProtocol::Pedestal, TraceStalk>,
}

impl<TraceProtocol: IsTraceProtocol> TraceSynchrotronEntry<TraceProtocol> {
    pub fn new(var_deps: SmallVec<[ItemPathIdInterface; 2]>, view_data: TraceViewData) -> Self {
        Self {
            var_deps,
            view_data,
            subtrace_ids: None,
            assoc_trace_ids_shown: Default::default(),
            expanded: false,
            stalks: Default::default(),
        }
    }

    pub fn var_deps(&self) -> &[ItemPathIdInterface] {
        &self.var_deps
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

    pub(crate) fn has_stalk(&self, pedestal: &TraceProtocol::Pedestal) -> bool {
        self.stalks.contains_key(pedestal)
    }

    pub fn stalk(&self, pedestal: &TraceProtocol::Pedestal) -> &TraceStalk {
        &self.stalks[pedestal]
    }

    pub(super) fn toggle_expansion(&mut self) {
        self.expanded = !self.expanded
    }

    pub(super) fn toggle_assoc_traces(&mut self, assoc_trace_id: TraceId) {
        self.assoc_trace_ids_shown.toggle(assoc_trace_id)
    }

    pub fn assoc_trace_ids(&self) -> &[TraceId] {
        self.assoc_trace_ids_shown.as_ref()
    }

    pub(crate) fn cache_subtraces(&mut self, subtrace_ids: Vec<TraceId>) {
        assert!(self.subtrace_ids.is_none());
        self.subtrace_ids = Some(subtrace_ids)
    }

    pub(super) fn cache_stalk(&mut self, pedestal: TraceProtocol::Pedestal, stalk: TraceStalk) {
        // self.stalks.get_value_mut_or_insert_with(pedestal, f);
        debug_assert!(self.stalks.insert(pedestal, stalk).is_none());
    }
}
