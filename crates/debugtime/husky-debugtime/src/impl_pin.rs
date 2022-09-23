use super::*;

impl HuskyDebugtime {
    pub fn toggle_pin(
        &mut self,
        trace_id: TraceId,
    ) -> HuskyDebugtimeTakeChangeM<(
        Vec<(FigureCanvasKey, FigureCanvasData)>,
        Vec<(FigureControlKey, FigureControlData)>,
    )> {
        self.state.pins.toggle(trace_id);
        self.update_figure_canvases()?;
        self.update_figure_controls()?;
        HuskyDebugtimeTakeChangeM::Ok((todo!(), todo!()))
    }
}
