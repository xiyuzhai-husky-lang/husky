use super::*;

impl TreeContext {
    pub(crate) fn trace(&self, trace_id: TraceId) -> Rc<TraceProps> {
        let trace = self.trace_nodes.borrow()[trace_id.0].trace.clone();
        assert!(trace.id == trace_id);
        trace
    }

    pub(crate) fn get_subtraces(&self, focus: &Focus, trace_id: TraceId) -> &[TraceId] {
        todo!() // return self.subtraces_dict.get(
                //     focus.gen_subtraces_key(trace),
                //     () => `failed to get subtraces for trace ${JSON.stringify(trace)}`
                // );
    }

    pub(crate) fn is_subtraces_cached(focus: Focus, trace: &TraceProps) -> bool {
        todo!()
        // return self.subtraces_dict.has(focus.gen_subtraces_key(trace));
    }

    pub(crate) fn receive_subtraces(
        &mut self,
        trace_id: TraceId,
        effective_opt_input_id: Option<TraceId>,
        subtraces: Vec<Rc<TraceProps>>,
    ) {
        todo!() // self.cache_traces(subtraces);
                // self.subtraces_dict.insert_new(
                //     gen_subtraces_key(effective_opt_input_id, trace_id),
                //     subtraces
                // );
    }

    fn cache_traces(&mut self, new_trace_nodes: Vec<TraceNodeData>) {
        let trace_nodes = &mut self.trace_nodes.borrow_mut();
        let new_len = trace_nodes.len() + new_trace_nodes.len();
        trace_nodes.reserve(new_len);
        for trace_node in new_trace_nodes {
            assert_eq!(trace_node.trace.id.0, trace_nodes.len());
            trace_nodes.push(trace_node.into())
        }
    }

    fn set_trace_stalk(&mut self, trace_id: TraceId, input_id: usize, stalk: TraceStalk) {
        assert!(self
            .trace_stalks
            .insert((trace_id, Some(input_id)), stalk)
            .is_none());
        // self.trace_stalk_stores_table[trace_id][input_id].set(stalk);
    }

    //  fn   get_trace_stalk_store(
    //         trace_id: TraceId,
    //         input_id: usize,
    //         make_request: ()
    //     ) {
    //         return self.trace_stalk_store_map.get_store(
    //             trace_id,
    //             input_id,
    //             make_request
    //         );
    //     }

    // fn print_state() {
    //     console.log("trace cache:");
    //     console.log("    traces");
    //     self.traces.print_state(8);
    // }
}

// function gen_subtraces_key(
//     effective_opt_input_id_for_subtraces: usize | null,
//     trace_id: TraceId
// ): string {
//     return `${effective_opt_input_id_for_subtraces}:${trace_id}`;
// }

// function gen_subtraces_effective_opt_input_id(
//     opt_input_id: usize | null,
//     trace: Trace
// ): usize | null {
//     throw new Error("TODO");
// }
