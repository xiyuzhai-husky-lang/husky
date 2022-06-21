mod impl_control;
mod impl_storage;

use super::*;
use impl_control::*;
use impl_storage::*;

#[derive(Debug, Default)]
pub struct TraceContext {
    pub trace_nodes: RefCell<Vec<TraceNodeState>>,
    pub subtrace_ids_map: RefCell<HashMap<SubtracesKey, Vec<TraceId>>>,
    pub trace_stalks: RefCell<HashMap<TraceStalkKey, Rc<TraceStalkData>>>,
    pub root_trace_ids: Signal<Vec<TraceId>>,
    pub opt_active_trace_id: Signal<Option<TraceId>>,
    pub trace_listing: Signal<Vec<TraceId>>,
}

#[derive(Debug)]
pub struct TraceNodeState {
    data: Rc<TraceData>,
    expanded: Rc<Signal<bool>>,
    shown: Rc<Signal<bool>>,
}

impl From<TraceNodeData> for TraceNodeState {
    fn from(node_data: TraceNodeData) -> Self {
        TraceNodeState {
            data: Rc::new(node_data.raw_data.into()),
            expanded: Rc::new(Signal::new(node_data.expanded)),
            shown: Rc::new(Signal::new(node_data.shown)),
        }
    }
}

impl TraceContext {
    pub(super) fn init(&self, attention: &Attention, init_data: TraceInitState) {
        *self.trace_nodes.borrow_mut(file!(), line!()) = init_data
            .trace_nodes
            .into_iter()
            .map(|node| node.into())
            .collect();
        *self.subtrace_ids_map.borrow_mut(file!(), line!()) =
            init_data.subtrace_ids_map.into_iter().collect();
        log::info!(
            "init_data.trace_stalks.len()={}",
            init_data.trace_stalks.len()
        );
        *self.trace_stalks.borrow_mut(file!(), line!()) = init_data
            .trace_stalks
            .into_iter()
            .map(|(key, raw_data)| -> (_, Rc<TraceStalkData>) { (key, Rc::new(raw_data.into())) })
            .collect();
        self.root_trace_ids.set(init_data.root_trace_ids);
        self.opt_active_trace_id.set(init_data.opt_active_trace_id);
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
        if (self.expanded(trace_id)) {
            for subtrace_id in self.subtrace_ids(attention, trace_id) {
                self.update_trace_listing_dfs(attention, subtrace_id, trace_listing);
            }
        }
    }

    fn add_associated_traces(
        &self,
        attention: &Attention,
        trace_id: TraceId,
        trace_listing: &mut Vec<TraceId>,
    ) {
        let trace = self.trace(trace_id);
        for line in &trace.lines {
            for token in &line.tokens {
                if let Some(associated_trace_id) = token.opt_associated_trace_id {
                    if (self.shown(associated_trace_id)) {
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
