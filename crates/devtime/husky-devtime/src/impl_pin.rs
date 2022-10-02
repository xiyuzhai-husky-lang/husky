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
        self.update()?;
        let change = self.take_change()?;
        HuskyDebugtimeTakeChangeM::Ok((
            change.figure_canvases.opt_new_entries().unwrap_or_default(),
            change.figure_controls.opt_new_entries().unwrap_or_default(),
        ))
    }
}
