use super::*;

impl DebuggerContext {
    pub(super) fn toggle_pin(&'static self, trace_id: TraceId) {
        let pins = self.figure_context.pins;
        let trace = self.trace_context.trace_data(trace_id);
        let pinned = pins.get().has(trace_id);
        let restriction = self.restriction_context.restriction.get();
        let needs_figure_canvas_data =
            !pinned && (self.needs_figure_canvas_data(Some(trace_id), &restriction));
        let needs_figure_control_data =
            !pinned && self.needs_figure_control_data(Some(trace_id), &restriction);
        let needs_response = needs_figure_canvas_data || needs_figure_control_data;

        self.ws.send_message(
            HuskyTracerGuiMessageVariant::TogglePin {
                trace_id,
                needs_figure_canvas_data,
                needs_figure_control_data,
            },
            if needs_response {
                Some(Box::new(move |response| match response.variant {
                    HuskyTracerServerMessageVariant::TogglePin {
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
                        self.figure_context.did_toggle_pin(trace_id);
                    }
                    HuskyTracerServerMessageVariant::TogglePinWithError { .. } => todo!(),
                    _ => panic!("unexpected response {:?}", response),
                }))
            } else {
                {
                    self.figure_context.did_toggle_pin(trace_id);
                    None
                }
            },
        )
    }
}
