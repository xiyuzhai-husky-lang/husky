use super::*;

impl DeveloperGuiContext {
    fn expansion_signal(&self, trace_id: TraceId) -> &'static Signal<bool> {
        self.trace_nodes.borrow(file!(), line!())[trace_id.raw()].expansion
    }

    pub(super) fn toggle_expansion(&'static self, trace_id: TraceId) {
        let expansion = self.expansion_signal(trace_id);
        if expansion.cget() {
            expansion.set(false)
        } else {
            let trace_kind = self.trace_kind(trace_id);
            let key = SubtracesKey::new(
                trace_kind,
                trace_id,
                self.presentation_signal.get().opt_sample_id(),
            );
            if self
                .subtrace_ids_map
                .borrow(file!(), line!())
                .contains_key(&key)
            {
                self.ws.send_message(
                    HuskyTracerGuiMessageVariant::ToggleExpansion { trace_id },
                    None,
                );
                expansion.set(true)
            } else {
                self.ws.send_message(
                    HuskyTracerGuiMessageVariant::ToggleExpansion { trace_id },
                    Some(Box::new(move |message| match message.variant {
                        HuskyTracerServerMessageVariant::ToggleExpansion {
                            new_traces,
                            subtrace_ids,
                            trace_stalks,
                            trace_stats,
                        } => {
                            self.receive_subtraces(key, self.alloc_value(subtrace_ids));
                            self.receive_traces(
                                new_traces
                                    .into_iter()
                                    .map(|trace| TraceNodeState::from_data(self.scope, trace)),
                            );
                            self.receive_trace_stalks(
                                trace_stalks
                                    .into_iter()
                                    .map(|(k, v)| (k, self.alloc_value(v))),
                            );
                            self.receive_trace_stats(
                                trace_stats
                                    .into_iter()
                                    .map(|(k, v)| (k, v.map(|v| self.alloc_value(v)))),
                            );
                            expansion.set(true)
                        }
                        _ => panic!(),
                    })),
                )
            }
        }
    }
}
