mod trace_cache;
mod trace_control;

use super::*;
use trace_cache::*;
use trace_control::*;

#[derive(Debug, Default)]
pub struct TraceContext {
    storage: TraceStorage,
    control: TraceControl,
    trace_listing: Signal<Vec<TraceId>>,
}

impl TraceContext {
    pub(super) fn init(&mut self, focus: &Focus, init_data: TraceInitState) {
        self.storage.init(
            init_data.traces,
            init_data.root_traces,
            init_data.subtraces_list,
        );
        let active_trace = init_data
            .active_trace_id
            .map(|trace_id| self.storage.get_trace(trace_id).unwrap());
        self.control
            .init(active_trace, init_data.expansions, init_data.showns);
        self.update_trace_listing(focus);
    }

    fn active_trace(&self) -> Option<Rc<TraceProps>> {
        return (*self.control.active_trace_store.get()).clone();
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
        for trace in &self.storage.root_traces {
            self.update_trace_listing_dfs(focus, trace, &mut trace_listing);
        }
        self.trace_listing.set(trace_listing);
    }

    fn update_trace_listing_dfs(
        &self,
        focus: &Focus,
        trace: &TraceProps,
        trace_listing: &mut Vec<TraceId>,
    ) {
        trace_listing.push(trace.id);
        self.add_associated_traces(focus, trace.id, trace_listing);
        if (self.control.is_expanded(trace.id)) {
            for trace in self.storage.get_subtraces(focus, trace) {
                self.update_trace_listing_dfs(focus, trace, trace_listing);
            }
        }
    }

    fn add_associated_traces(
        &self,
        focus: &Focus,
        trace_id: TraceId,
        trace_listing: &mut Vec<TraceId>,
    ) {
        let trace = self.storage.get_trace(trace_id).unwrap();
        for line in &trace.lines {
            for token in &line.tokens {
                if let Some(associated_trace_id) = token.opt_associated_trace_id {
                    if (self.control.is_shown(associated_trace_id)) {
                        let associated_trace = self.storage.get_trace(associated_trace_id).unwrap();
                        self.update_trace_listing_dfs(focus, &associated_trace, trace_listing);
                    }
                }
            }
        }
    }

    fn did_toggle_expansion(&mut self, focus: &Focus, trace_id: TraceId) {
        self.control.did_toggle_expansion(trace_id);
        self.update_trace_listing(focus);
    }

    fn did_toggle_show(&mut self, focus: &Focus, trace_id: TraceId) {
        self.control.did_toggle_show(trace_id);
        self.update_trace_listing(focus);
    }

    // fn  print_state() {
    //     throw new Error("todo");
    //     // self.user_state.print_state();
    //     // self.trace_cache.print_state();
    // }
}
