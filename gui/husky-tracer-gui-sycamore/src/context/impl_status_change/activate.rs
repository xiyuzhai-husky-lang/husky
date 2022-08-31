use super::*;

impl DebuggerContext {
    pub(crate) fn activate(&'static self, new_active_trace_id: TraceId) {
        let restriction = self.restriction_context.restriction.get();
        let trace = self.trace_context.trace_data(new_active_trace_id);
        let needs_figure_canvases =
            self.needs_figure_canvases(Some(new_active_trace_id), &restriction);
        let needs_figure_controls =
            self.needs_figure_controls(Some(new_active_trace_id), &restriction);
        let needs_response = needs_figure_canvases || needs_figure_controls;
        self.ws.send_message(
            HuskyTracerGuiMessageVariant::Activate {
                trace_id: new_active_trace_id,
                needs_figure_canvases,
                needs_figure_controls,
            },
            if needs_response {
                Some(Box::new(move |response| match response.variant {
                    HuskyTracerServerMessageVariant::Activate {
                        new_figure_canvases,
                        new_figure_controls,
                    } => {
                        self.figure_context.receive_figure_canvases(
                            self.scope,
                            new_figure_canvases
                                .into_iter()
                                .map(|(k, v)| (k, self.alloc_value(v))),
                        );
                        self.figure_context
                            .receive_figure_controls(self.scope, new_figure_controls.into_iter());
                        self.trace_context.did_activate(new_active_trace_id);
                    }
                    HuskyTracerServerMessageVariant::ActivateWithError { .. } => todo!(),
                    _ => panic!("unexpected response {:?}", response),
                }))
            } else {
                {
                    self.trace_context.did_activate(new_active_trace_id);
                    None
                }
            },
        );
    }
}
