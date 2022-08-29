use super::*;

impl TraceContext {
    pub(crate) fn trace_data(&self, trace_id: TraceId) -> &'static TraceData {
        let trace_data = self.trace_nodes.borrow(file!(), line!())[trace_id.0].data;
        assert!(trace_data.id == trace_id);
        trace_data
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
    ) -> &'static TraceStalk {
        let key = TraceStalkKey::from_trace_data(
            sample_id,
            &self.trace_nodes.borrow(file!(), line!())[trace_id.0].data,
        );
        if let Some(trace_stalk) = self.trace_stalks.borrow(file!(), line!()).get(&key) {
            trace_stalk
        } else {
            log::info!(
                "self.trace_stalks = {:?}",
                self.trace_stalks.borrow(file!(), line!())
            );
            let trace = self.trace_nodes.borrow(file!(), line!())[trace_id.0].data;
            log::info!("trace: {:?}", trace);
            panic!("no trace stalk for key {:?}", key);
        }
    }

    pub(crate) fn opt_trace_stats(
        &self,
        trace_id: TraceId,
        restriction: &Restriction,
    ) -> Option<&'static TraceStats> {
        let key = TraceStatsKey {
            trace_id,
            partitions: restriction.partitions().clone(),
        };
        if let Some(opt_trace_stats) = self.trace_statss.borrow(file!(), line!()).get(&key) {
            opt_trace_stats.clone()
        } else {
            log::info!(
                "self.trace_statss = {:?}",
                self.trace_statss.borrow(file!(), line!())
            );
            let trace = self.trace_nodes.borrow(file!(), line!())[trace_id.0].data;
            log::info!("trace: {:?}", trace);
            panic!("no opt trace stats for key {:?}", key);
        }
    }

    pub(crate) fn opt_active_trace(&self) -> Option<&'static TraceData> {
        self.opt_active_trace_id
            .cget()
            .map(|trace_id| self.trace_data(trace_id))
    }

    pub(crate) fn subtrace_ids(
        &self,
        trace_id: TraceId,
        opt_sample_id: Option<SampleId>,
    ) -> &'static [TraceId] {
        let trace_data = self.trace_data(trace_id);
        if !trace_data.has_subtraces(opt_sample_id.is_some()) {
            return &[];
        }
        let subtraces_key = SubtracesKey::new(trace_data.kind, trace_id, opt_sample_id);
        if let Some(subtrace_ids) = self
            .subtrace_ids_map
            .borrow(file!(), line!())
            .get(&subtraces_key)
        {
            subtrace_ids
        } else {
            let trace = self.trace_nodes.borrow(file!(), line!())[trace_id.0].data;
            log::info!("trace: {:?}", trace);
            panic!("no subtraces for key {:?}", subtraces_key);
        }
    }

    pub(crate) fn is_subtraces_cached(
        &self,
        trace_id: TraceId,
        opt_sample_id: Option<SampleId>,
    ) -> bool {
        self.subtrace_ids_map
            .borrow(file!(), line!())
            .contains_key(&SubtracesKey::new(
                self.trace_kind(trace_id),
                trace_id,
                opt_sample_id,
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
        new_trace_stalks: impl Iterator<Item = (TraceStalkKey, &'static TraceStalk)>,
    ) {
        let mut trace_stalks = self.trace_stalks.borrow_mut(file!(), line!());
        for (key, data) in new_trace_stalks {
            assert!(trace_stalks.insert(key, data).is_none());
        }
    }

    pub(crate) fn receive_trace_stats(
        &self,
        new_trace_stats: impl Iterator<Item = (TraceStatsKey, Option<&'static TraceStats>)>,
    ) {
        let mut trace_stats = self.trace_statss.borrow_mut(file!(), line!());
        for (key, data) in new_trace_stats {
            assert!(trace_stats.insert(key, data).is_none());
        }
    }

    fn set_trace_stalk(&self, trace_id: TraceId, input_id: usize, stalk: &'static TraceStalk) {
        assert!(self
            .trace_stalks
            .borrow_mut(file!(), line!())
            .insert(todo!(), stalk)
            .is_none());
    }
}
