use super::*;

#[derive(Debug, Default)]
pub struct TraceStorage {
    pub(super) traces: HashMap<TraceId, Rc<TraceProps>>,
    pub(super) subtraces_dict: HashMap<(TraceId, Option<usize>), Vec<Rc<TraceProps>>>,
    pub(super) trace_stalks: HashMap<(TraceId, Option<usize>), TraceStalk>,
    pub(super) root_traces: Vec<Rc<TraceProps>>,
}

impl TraceStorage {
    pub(super) fn init(
        &mut self,
        traces: Vec<TraceProps>,
        root_traces: Vec<TraceId>,
        subtraces_list: HashMap<(TraceId, Option<usize>), Vec<TraceId>>,
    ) {
        self.init_traces(traces);
        self.init_root_traces_stores(root_traces);
        self.init_subtraces_map(subtraces_list);
    }

    fn init_traces(&mut self, traces: Vec<TraceProps>) {
        self.traces = traces
            .into_iter()
            .map(|trace| (trace.id, Rc::new(trace)))
            .collect();
    }

    fn init_root_traces_stores(&mut self, root_trace_ids: Vec<TraceId>) {
        self.root_traces = root_trace_ids
            .into_iter()
            .map(|id| self.traces[&id].clone())
            .collect();
    }

    fn init_subtraces_map(
        &mut self,
        subtraces_list: HashMap<(TraceId, Option<usize>), Vec<TraceId>>,
    ) {
        self.subtraces_dict = subtraces_list
            .into_iter()
            .map(|(k, v)| {
                let traces: Vec<Rc<TraceProps>> = v
                    .into_iter()
                    .map(|trace_id| self.traces[&trace_id].clone())
                    .collect();
                (k, traces)
            })
            .collect();
    }

    pub(super) fn get_trace(&self, trace_id: TraceId) -> Option<Rc<TraceProps>> {
        self.traces.get(&trace_id).map(|trace| trace.clone())
    }

    pub(super) fn get_subtraces(&self, focus: &Focus, trace: &TraceProps) -> &[Rc<TraceProps>] {
        todo!() // return self.subtraces_dict.get(
                //     focus.gen_subtraces_key(trace),
                //     () => `failed to get subtraces for trace ${JSON.stringify(trace)}`
                // );
    }

    pub(super) fn is_subtraces_cached(focus: Focus, trace: &TraceProps) -> bool {
        todo!()
        // return self.subtraces_dict.has(focus.gen_subtraces_key(trace));
    }

    pub(super) fn receive_subtraces(
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

    fn cache_traces(&mut self, traces: Vec<TraceProps>) {
        traces.into_iter().for_each(|trace| self.cache_trace(trace))
    }

    fn cache_trace(&mut self, trace: TraceProps) {
        assert!(self.traces.insert(trace.id, Rc::new(trace)).is_none());
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
