use crate::*;

impl TraceContext {
    pub fn filter_immediate_traces<T>(
        &self,
        opt_sample_id: Option<SampleId>,
        f: impl Fn(&TraceData) -> Option<T>,
    ) -> Vec<T> {
        let mut shown_traces = vec![];
        for trace_id in self.root_trace_ids.get().iter() {
            self.filter_immediate_traces_dfs(*trace_id, opt_sample_id, &f, &mut shown_traces)
        }
        shown_traces
    }
    pub fn filter_immediate_traces_dfs<T>(
        &self,
        trace_id: TraceId,
        opt_sample_id: Option<SampleId>,
        f: &impl Fn(&TraceData) -> Option<T>,
        shown_traces: &mut Vec<T>,
    ) {
        let trace_data = &self.trace_data(trace_id);
        if let Some(t) = f(trace_data) {
            shown_traces.push(t);
        }
        for associated_trace_id in trace_data.associated_trace_ids() {
            self.filter_immediate_traces_dfs(associated_trace_id, opt_sample_id, f, shown_traces)
        }
        if self.is_expanded(trace_id) {
            for subtrace_id in self.subtrace_ids(trace_id, opt_sample_id) {
                self.filter_immediate_traces_dfs(*subtrace_id, opt_sample_id, f, shown_traces)
            }
        }
    }

    pub fn for_all_immediate_traces(
        &self,
        opt_sample_id: Option<SampleId>,
        predicate: impl Fn(&TraceData) -> bool,
    ) -> bool {
        for trace_id in self.root_trace_ids.get().iter() {
            if !self.for_all_immediate_traces_dfs(*trace_id, opt_sample_id, &predicate) {
                return false;
            }
        }
        true
    }

    fn for_all_immediate_traces_dfs<F>(
        &self,
        trace_id: TraceId,
        opt_sample_id: Option<SampleId>,
        predicate: &F,
    ) -> bool
    where
        F: Fn(&TraceData) -> bool,
    {
        let trace_data = &self.trace_data(trace_id);
        if !predicate(trace_data) {
            return false;
        }
        for associated_trace_id in trace_data.associated_trace_ids() {
            if !self.for_all_immediate_traces_dfs::<F>(
                associated_trace_id,
                opt_sample_id,
                &predicate,
            ) {
                return false;
            }
        }
        if self.is_expanded(trace_id) {
            for subtrace_id in self.subtrace_ids(trace_id, opt_sample_id) {
                if !self.for_all_immediate_traces_dfs::<F>(*subtrace_id, opt_sample_id, &predicate)
                {
                    return false;
                }
            }
        }
        true
    }
}
