use super::*;

impl TraceContext {
    pub(crate) fn trace(&self, trace_id: TraceId) -> &'static TraceData {
        let trace = self.trace_nodes.borrow(file!(), line!())[trace_id.0].data;
        assert!(trace.id == trace_id);
        trace
    }

    pub(crate) fn trace_kind(&self, trace_id: TraceId) -> TraceKind {
        self.trace_nodes.borrow(file!(), line!())[trace_id.0]
            .data
            .kind
    }

    pub(crate) fn trace_stalk(
        &self,
        sample_id: SampleId,
        trace_id: TraceId,
    ) -> &'static TraceStalkData {
        let key = TraceStalkKey::from_trace_data(
            sample_id,
            &self.trace_nodes.borrow(file!(), line!())[trace_id.0].data,
        );
        // self.trace_stalks.borrow(file!(), line!())[&key]
        if let Some(trace_stalk) = self.trace_stalks.borrow(file!(), line!()).get(&key) {
            trace_stalk
        } else {
            log::info!("{:?}", key);
            let trace = self.trace_nodes.borrow(file!(), line!())[trace_id.0].data;
            log::info!("trace: {:?}", trace);
            panic!()
        }
    }

    pub(crate) fn opt_active_trace(&self) -> Option<&'static TraceData> {
        self.opt_active_trace_id
            .cget()
            .map(|trace_id| self.trace(trace_id))
    }

    pub(crate) fn subtrace_ids(&self, attention: &Attention, trace_id: TraceId) -> Vec<TraceId> {
        self.subtrace_ids_map.borrow(file!(), line!())
            [&SubtracesKey::new(attention, self.trace_kind(trace_id), trace_id)]
            .to_vec()
    }

    pub(crate) fn is_subtraces_cached(&self, attention: &Attention, trace_id: TraceId) -> bool {
        self.subtrace_ids_map
            .borrow(file!(), line!())
            .contains_key(&SubtracesKey::new(
                attention,
                self.trace_kind(trace_id),
                trace_id,
            ))
    }

    pub(crate) fn receive_subtraces(&self, key: SubtracesKey, subtrace_ids: &'static [TraceId]) {
        self.subtrace_ids_map
            .borrow_mut(file!(), line!())
            .insert(key, subtrace_ids);
    }

    pub(crate) fn receive_traces(&self, new_trace_nodes: impl Iterator<Item = TraceNodeState>) {
        let trace_nodes = &mut self.trace_nodes.borrow_mut(file!(), line!());
        for trace_node in new_trace_nodes {
            assert_eq!(trace_node.data.id.0, trace_nodes.len());
            trace_nodes.push(trace_node);
        }
    }
    pub(crate) fn receive_trace_stalks(
        &self,
        new_trace_stalks: impl Iterator<Item = (TraceStalkKey, &'static TraceStalkData)>,
    ) {
        log::info!("receive trace stalks");
        let mut trace_stalks = self.trace_stalks.borrow_mut(file!(), line!());
        for (key, raw_data) in new_trace_stalks {
            assert!(trace_stalks.insert(key, raw_data).is_none());
        }
    }

    fn set_trace_stalk(&self, trace_id: TraceId, input_id: usize, stalk: &'static TraceStalkData) {
        assert!(self
            .trace_stalks
            .borrow_mut(file!(), line!())
            .insert(todo!(), stalk)
            .is_none());
        // (trace_id, Some(input_id))
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
