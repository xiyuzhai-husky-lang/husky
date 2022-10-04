mod impl_query;
mod impl_storage;
mod utils;

use super::*;
use impl_query::*;
use impl_storage::*;

#[derive(Debug)]
pub struct TraceNodeState {
    pub(super) data: &'static TraceData,
    pub(super) expansion: &'static Signal<bool>,
    pub(super) shown: &'static Signal<bool>,
}

impl TraceNodeState {
    pub(super) fn from_data(scope: Scope<'static>, node_data: TraceNodeData) -> Self {
        TraceNodeState {
            data: create_static_ref(scope, node_data.trace_data),
            expansion: create_static_signal(scope, node_data.expanded),
            shown: create_static_signal(scope, node_data.shown),
        }
    }
}

impl DeveloperGuiContext {
    pub(super) fn init_trace_context<'a>(
        &'static self,
        trace_nodes: Vec<TraceNodeState>,
        trace_stalks: HashMap<TraceStalkKey, &'static TraceStalk>,
        trace_statss: HashMap<TraceStatsKey, Option<&'static TraceStats>>,
        subtrace_ids_map: HashMap<SubtracesKey, &'static [TraceId]>,
        root_trace_ids: Vec<TraceId>,
        opt_active_trace_id: Option<TraceId>,
        opt_sample_id: Option<SampleId>,
    ) {
        *self.trace_nodes.borrow_mut(file!(), line!()) = trace_nodes;
        *self.subtrace_ids_map.borrow_mut(file!(), line!()) = subtrace_ids_map;
        *self.trace_stalks.borrow_mut(file!(), line!()) = trace_stalks;
        *self.trace_statss.borrow_mut(file!(), line!()) = trace_statss;
        self.root_trace_ids_signal.set(root_trace_ids);
        self.update_trace_listing(opt_sample_id);
    }

    pub(crate) fn root_trace_ids_signal(&self) -> &'static ReadSignal<Vec<TraceId>> {
        &self.root_trace_ids_signal
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

    fn update_trace_listing(&self, opt_sample_id: Option<SampleId>) {
        let mut trace_listing: Vec<TraceId> = vec![];
        for trace_id in &*self.root_trace_ids_signal.get() {
            self.update_trace_listing_dfs(*trace_id, opt_sample_id, &mut trace_listing);
        }
        self.trace_listing.set(trace_listing);
    }

    fn update_trace_listing_dfs(
        &self,
        trace_id: TraceId,
        opt_sample_id: Option<SampleId>,
        trace_listing: &mut Vec<TraceId>,
    ) {
        trace_listing.push(trace_id);
        self.add_associated_traces(trace_id, trace_listing, opt_sample_id);
        if (self.is_expanded(trace_id)) {
            for subtrace_id in self.subtrace_ids(trace_id, opt_sample_id) {
                self.update_trace_listing_dfs(*subtrace_id, opt_sample_id, trace_listing);
            }
        }
    }

    fn add_associated_traces(
        &self,
        trace_id: TraceId,
        trace_listing: &mut Vec<TraceId>,
        opt_sample_id: Option<SampleId>,
    ) {
        let trace = self.trace_data(trace_id);
        for line in &trace.lines {
            for token in &line.tokens {
                if let Some(associated_trace_id) = token.opt_associated_trace_id {
                    if (self.is_shown(associated_trace_id)) {
                        self.update_trace_listing_dfs(
                            associated_trace_id,
                            opt_sample_id,
                            trace_listing,
                        );
                    }
                }
            }
        }
    }
}
