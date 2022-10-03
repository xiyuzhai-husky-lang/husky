use super::*;

impl DebuggerContext {
    pub(super) fn toggle_pin(&'static self, trace_id: TraceId) {
        let pins = self.pins;
        let trace = self.trace_context.trace_data(trace_id);
        let pinned = pins.get().has(trace_id);
        let restriction = self.restriction_context.presentation.get();
        let needs_figure_canvases =
            !pinned && (self.needs_figure_canvases(Some(trace_id), &restriction));
        let needs_figure_controls =
            !pinned && self.needs_figure_controls(Some(trace_id), &restriction);
        let needs_response = needs_figure_canvases || needs_figure_controls;

        self.ws.send_message(
            HuskyTracerGuiMessageVariant::TogglePin {
                trace_id,
                needs_figure_canvases,
                needs_figure_controls,
            },
            if needs_response {
                Some(Box::new(move |response| match response.variant {
                    HuskyTracerServerMessageVariant::TogglePin {
                        new_figure_canvases,
                        new_figure_controls,
                    } => {
                        self.receive_figure_canvases(
                            self.scope,
                            new_figure_canvases
                                .into_iter()
                                .map(|(k, v)| (k, self.alloc_value(v))),
                        );
                        self.receive_figure_controls(self.scope, new_figure_controls.into_iter());
                        self.did_toggle_pin(trace_id);
                    }
                    HuskyTracerServerMessageVariant::TogglePinWithError { .. } => todo!(),
                    _ => panic!("unexpected response {:?}", response),
                }))
            } else {
                {
                    self.did_toggle_pin(trace_id);
                    None
                }
            },
        )
    }
}
