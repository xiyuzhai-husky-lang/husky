use super::*;

impl HuskyDebugtime {
    pub fn toggle_pin(
        &mut self,
        trace_id: TraceId,
    ) -> HuskyDebugtimeStageChangeM<(
        Vec<(FigureCanvasKey, FigureCanvasData)>,
        Vec<(FigureControlKey, FigureControlData)>,
    )> {
        self.state.pins.toggle(trace_id);
        self.update_figure_canvases()?;
        self.update_figure_controls()?;
        HuskyDebugtimeStageChangeM::Ok((todo!(), todo!()))
    }
}
