use super::*;

impl Tracetime {
    pub fn toggle_pin(
        &mut self,
        trace_id: TraceId,
    ) -> TracetimeUpdatedM<(
        Vec<(FigureCanvasKey, FigureCanvasData)>,
        Vec<(FigureControlKey, FigureControlData)>,
    )> {
        self.state.pins.toggle(trace_id);
        TracetimeUpdatedM::Ok((
            self.update_figure_canvases()?,
            self.update_figure_controls()?,
        ))
    }
}
