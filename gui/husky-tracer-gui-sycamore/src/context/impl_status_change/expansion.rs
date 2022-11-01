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
            let needs_response = !self
                .subtrace_ids_map
                .borrow(file!(), line!())
                .contains_key(&key);
            self.ws.send_message(
                HuskyTracerGuiMessageVariant::ToggleExpansion { trace_id },
                needs_response,
            );
            if !needs_response {
                expansion.set(true)
            }
        }
    }
}
