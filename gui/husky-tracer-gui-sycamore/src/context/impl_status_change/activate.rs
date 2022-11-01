use super::*;

impl DeveloperGuiContext {
    pub(crate) fn activate(&'static self, new_active_trace_id: TraceId) {
        if self.opt_active_trace_id() == Some(new_active_trace_id) {
            // do nothing if already activated
            // without this toggle expansion doesn't work
            return;
        }
        let trace_data = self.trace_data(new_active_trace_id);
        let presentation = self
            .presentation_signal()
            .get()
            .activate_trace_out_of_place(trace_data);
        let needs_figure_canvases =
            self.needs_figure_canvases(Some(new_active_trace_id), &presentation);
        let needs_figure_controls =
            self.needs_figure_controls(Some(new_active_trace_id), &presentation);
        let needs_response = needs_figure_canvases || needs_figure_controls;
        self.ws.try_apply_change(
            HuskyTracerGuiMessageVariant::Activate {
                trace_id: new_active_trace_id,
                needs_figure_canvases,
                needs_figure_controls,
            },
            needs_response,
            || self.did_activate(presentation),
        );
    }

    fn did_activate(&'static self, presentation: Presentation) {
        self.presentation_signal.set(presentation)
    }
}
