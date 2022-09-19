use super::*;

impl Debugtime {
    pub fn toggle_pin(
        &mut self,
        trace_id: TraceId,
    ) -> DebugtimeUpdatedM<(
        Vec<(FigureCanvasKey, FigureCanvasData)>,
        Vec<(FigureControlKey, FigureControlData)>,
    )> {
        self.state.pins.toggle(trace_id);
        DebugtimeUpdatedM::Ok((
            self.update_figure_canvases()?,
            self.update_figure_controls()?,
        ))
    }
}
