use super::*;

impl HuskyTraceTime {
    pub fn toggle_pin(
        &mut self,
        trace_id: TraceId,
    ) -> __VMResult<(
        Vec<(FigureCanvasKey, FigureCanvasData)>,
        Vec<(FigureControlKey, FigureControlData)>,
    )> {
        self.pins.toggle(trace_id);
        Ok((
            self.update_figure_canvases()?,
            self.update_figure_controls()?,
        ))
    }
}
