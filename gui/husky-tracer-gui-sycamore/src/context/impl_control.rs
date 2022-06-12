use super::*;

impl TracerContext {
    pub fn toggle_expansion_handler(&self, trace_id: TraceId) -> Rc<dyn Fn()> {
        let this = self.clone();
        Rc::new(move || this.toggle_expansion(trace_id))
    }

    fn toggle_expansion(&self, trace_id: TraceId) {
        let expansion = self.tree_context.expanded_signal(trace_id);
        if expansion.get_cloned() {
            expansion.set(false)
        } else {
            let focus = self.focus_context.focus_signal.get();
            let trace_kind = self.tree_context.trace_kind(trace_id);
            let key = SubtracesKey::new(&focus, trace_kind, trace_id);
            if self.tree_context.subtraces_map.contains_key(&key) {
                self.ws.send_message(
                    HuskyTracerGuiMessageVariant::ToggleExpansion {
                        trace_id,
                        opt_subtraces_key: None,
                    },
                    None,
                );
                expansion.set(true)
            } else {
                let this = self.clone();
                self.ws.send_message(
                    HuskyTracerGuiMessageVariant::ToggleExpansion {
                        trace_id,
                        opt_subtraces_key: Some(key.clone()),
                    },
                    Some(Box::new(move |message| match message.variant {
                        HuskyTracerServerMessageVariant::ToggleExpansion {
                            subtraces,
                            associated_traces,
                        } => {
                            this.tree_context.receive_subtraces(key, subtraces);
                            todo!()
                        }
                        _ => panic!(),
                    })),
                )
            }
        }
    }
}
