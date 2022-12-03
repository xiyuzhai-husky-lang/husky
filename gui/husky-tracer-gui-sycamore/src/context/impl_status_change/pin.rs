use super::*;

impl DeveloperGuiContext {
    pub(super) fn toggle_pin(&'static self, trace_id: TraceId) {
        let presentation = self.presentation_signal().get();
        let trace = self.trace_data(trace_id);
        let pinned = presentation.is_pinned(trace_id);
        let needs_figure_canvases =
            !pinned && (self.needs_figure_canvases(Some(trace_id), &presentation));
        let needs_figure_controls =
            !pinned && self.needs_figure_controls(Some(trace_id), &presentation);
        let needs_response = needs_figure_canvases || needs_figure_controls;

        self.ws.try_apply_change(
            HuskyTracerGuiMessageVariant::TogglePin {
                trace_id,
                needs_figure_canvases,
                needs_figure_controls,
            },
            needs_response,
            move || {
                let mut presentation: Presentation = (*presentation).clone();
                presentation.toggle_pin(trace_id);
                self.presentation_signal.set(presentation)
            },
        )
    }
}
