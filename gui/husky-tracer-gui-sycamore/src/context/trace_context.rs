mod impl_control;
mod impl_storage;

use super::*;
use impl_control::*;
use impl_storage::*;

#[derive(Debug, Default)]
pub struct TraceContext {
    pub trace_nodes: Vec<TraceNodeProps>,
    pub subtraces_map: HashMap<SubtracesKey, Vec<TraceId>>,
    pub trace_stalks: HashMap<(TraceId, Option<usize>), TraceStalk>,
    pub root_trace_ids: Vec<TraceId>,
    pub opt_active_trace_id: Signal<Option<TraceId>>,
    pub trace_listing: Signal<Vec<TraceId>>,
}

#[derive(Debug)]
pub struct TraceNodeProps {
    trace: Rc<TraceProps>,
    expanded: Signal<bool>,
    shown: Signal<bool>,
}

impl From<TraceNodeData> for TraceNodeProps {
    fn from(data: TraceNodeData) -> Self {
        TraceNodeProps {
            trace: Rc::new(data.trace),
            expanded: Signal::new(data.expansion),
            shown: Signal::new(data.shown),
        }
    }
}

impl TraceContext {
    pub(super) fn init(&mut self, focus: &Focus, init_data: TraceInitState) {
        self.trace_nodes = init_data
            .trace_nodes
            .into_iter()
            .map(|trace_node_data| trace_node_data.into())
            .collect();
        self.root_trace_ids = init_data.root_trace_ids;
        self.opt_active_trace_id = Signal::new(init_data.opt_active_trace_id);
        self.update_trace_listing(focus);
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

    fn update_trace_listing(&mut self, focus: &Focus) {
        let mut trace_listing: Vec<TraceId> = vec![];
        for trace_id in &self.root_trace_ids {
            self.update_trace_listing_dfs(focus, *trace_id, &mut trace_listing);
        }
        self.trace_listing.set(trace_listing);
    }

    fn update_trace_listing_dfs(
        &self,
        focus: &Focus,
        trace_id: TraceId,
        trace_listing: &mut Vec<TraceId>,
    ) {
        trace_listing.push(trace_id);
        self.add_associated_traces(focus, trace_id, trace_listing);
        if (self.expanded(trace_id)) {
            for subtrace_id in self.get_subtraces(focus, trace_id) {
                self.update_trace_listing_dfs(focus, *subtrace_id, trace_listing);
            }
        }
    }

    fn add_associated_traces(
        &self,
        focus: &Focus,
        trace_id: TraceId,
        trace_listing: &mut Vec<TraceId>,
    ) {
        let trace = self.trace(trace_id);
        for line in &trace.lines {
            for token in &line.tokens {
                if let Some(associated_trace_id) = token.opt_associated_trace_id {
                    if (self.shown(associated_trace_id)) {
                        self.update_trace_listing_dfs(focus, associated_trace_id, trace_listing);
                    }
                }
            }
        }
    }

    // fn  print_state() {
    //     throw new Error("todo");
    //     // self.user_state.print_state();
    //     // self.trace_cache.print_state();
    // }
}
