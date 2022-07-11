mod impl_query;
mod impl_storage;
mod utils;

use super::*;
use impl_query::*;
use impl_storage::*;

pub struct TraceContext {
    pub trace_nodes: RefCell<Vec<TraceNodeState>>,
    pub subtrace_ids_map: RefCell<HashMap<SubtracesKey, &'static [TraceId]>>,
    pub trace_stalks: RefCell<HashMap<TraceStalkKey, &'static TraceStalkData>>,
    pub root_trace_ids: &'static Signal<Vec<TraceId>>,
    pub opt_active_trace_id: &'static Signal<Option<TraceId>>,
    pub trace_listing: &'static Signal<Vec<TraceId>>,
}

#[derive(Debug)]
pub struct TraceNodeState {
    pub(super) data: &'static TraceData,
    pub(super) expansion: &'static Signal<bool>,
    pub(super) shown: &'static Signal<bool>,
    pub(super) pin: &'static Signal<bool>,
    pub(super) arrival: &'static Signal<bool>,
    pub(super) enter: &'static Signal<bool>,
}

impl TraceNodeState {
    pub(super) fn from_data(scope: Scope<'static>, node_data: TraceNodeData) -> Self {
        TraceNodeState {
            data: create_static_ref(scope, node_data.trace_data),
            expansion: create_static_signal(scope, node_data.expanded),
            shown: create_static_signal(scope, node_data.shown),
            pin: create_static_signal(scope, node_data.pin),
            arrival: create_static_signal(scope, node_data.arrival),
            enter: create_static_signal(scope, node_data.enter),
        }
    }
}

impl TraceContext {
    pub(super) fn new(scope: Scope<'static>) -> Self {
        Self {
            trace_nodes: Default::default(),
            subtrace_ids_map: Default::default(),
            trace_stalks: Default::default(),
            root_trace_ids: create_signal(scope, vec![]),
            opt_active_trace_id: create_signal(scope, None),
            trace_listing: create_signal(scope, vec![]),
        }
    }

    pub(super) fn init<'a>(
        &'static self,
        attention: &Attention,
        trace_nodes: Vec<TraceNodeState>,
        trace_stalks: HashMap<TraceStalkKey, &'static TraceStalkData>,
        subtrace_ids_map: HashMap<SubtracesKey, &'static [TraceId]>,
        root_trace_ids: Vec<TraceId>,
        opt_active_trace_id: Option<TraceId>,
    ) {
        *self.trace_nodes.borrow_mut(file!(), line!()) = trace_nodes;
        *self.subtrace_ids_map.borrow_mut(file!(), line!()) = subtrace_ids_map;
        *self.trace_stalks.borrow_mut(file!(), line!()) = trace_stalks;
        self.root_trace_ids.set(root_trace_ids);
        self.opt_active_trace_id.set(opt_active_trace_id);
        self.update_trace_listing(attention);
    }

    fn get_id_before(&self, trace_id: TraceId) -> Option<TraceId> {
        let trace_listing = self.trace_listing.get();
        let index = trace_listing
            .iter()
            .position(|candidate| *candidate == trace_id)
            .unwrap();
        if index == 0 {
            None
        } else {
            Some(trace_listing[index - 1])
        }
    }

    fn get_id_after(&mut self, target: TraceId) -> Option<TraceId> {
        let trace_listing = self.trace_listing.get();
        trace_listing
            .get(
                trace_listing
                    .iter()
                    .position(|trace_id| *trace_id == target)
                    .unwrap()
                    + 1,
            )
            .map(|id| *id)
    }

    fn update_trace_listing(&self, attention: &Attention) {
        let mut trace_listing: Vec<TraceId> = vec![];
        for trace_id in &*self.root_trace_ids.get() {
            self.update_trace_listing_dfs(attention, *trace_id, &mut trace_listing);
        }
        self.trace_listing.set(trace_listing);
    }

    fn update_trace_listing_dfs(
        &self,
        attention: &Attention,
        trace_id: TraceId,
        trace_listing: &mut Vec<TraceId>,
    ) {
        trace_listing.push(trace_id);
        self.add_associated_traces(attention, trace_id, trace_listing);
        if (self.is_expanded(trace_id)) {
            for subtrace_id in self.subtrace_ids(attention, trace_id) {
                self.update_trace_listing_dfs(attention, *subtrace_id, trace_listing);
            }
        }
    }

    fn add_associated_traces(
        &self,
        attention: &Attention,
        trace_id: TraceId,
        trace_listing: &mut Vec<TraceId>,
    ) {
        let trace = self.trace_data(trace_id);
        for line in &trace.lines {
            for token in &line.tokens {
                if let Some(associated_trace_id) = token.opt_associated_trace_id {
                    if (self.is_shown(associated_trace_id)) {
                        self.update_trace_listing_dfs(
                            attention,
                            associated_trace_id,
                            trace_listing,
                        );
                    }
                }
            }
        }
    }
}
