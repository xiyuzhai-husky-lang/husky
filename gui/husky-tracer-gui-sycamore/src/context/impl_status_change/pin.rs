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
        let needs_response = needs_figure_control_data || needs_figure_control_data;

        self.ws.send_message(
            HuskyTracerGuiMessageVariant::TogglePin {
                trace_id,
                needs_figure_canvas_data,
                needs_figure_control_data,
            },
            if needs_response {
                Some(Box::new(move |response| match response.variant {
                    HuskyTracerServerMessageVariant::TogglePin {
                        opt_figure_canvas_data,
                        opt_figure_control_data,
                    } => {
                        self.figure_context.set_opt_figure_data(
                            self.scope,
                            &trace,
                            &restriction,
                            opt_figure_canvas_data.map(|data| self.alloc_value(data)),
                            opt_figure_control_data,
                        );
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
