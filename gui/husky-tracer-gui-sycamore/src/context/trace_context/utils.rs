use crate::*;

impl TraceContext {
    pub fn for_all_expanded_traces(
        &self,
        restriction: &Restriction,
        predicate: impl Fn(&TraceData) -> bool,
    ) -> bool {
        for trace_id in self.root_trace_ids.get().iter() {
            if !self.for_all_expanded_traces_dfs(restriction, *trace_id, &predicate) {
                return false;
            }
        }
        true
    }

    fn for_all_expanded_traces_dfs<F>(
        &self,
        restriction: &Restriction,
        trace_id: TraceId,
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
            if !self.for_all_expanded_traces_dfs::<F>(restriction, associated_trace_id, &predicate)
            {
                return false;
            }
        }
        if self.is_expanded(trace_id) {
            for subtrace_id in self.subtrace_ids(restriction, trace_id) {
                if !self.for_all_expanded_traces_dfs::<F>(restriction, *subtrace_id, &predicate) {
                    return false;
                }
            }
        }
        true
    }
}
