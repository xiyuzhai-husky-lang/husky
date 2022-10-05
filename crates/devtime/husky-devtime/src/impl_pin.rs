use super::*;

impl HuskyDevtime {
    pub fn toggle_pin(
        &mut self,
        trace_id: TraceId,
    ) -> HuskyDevtimeTakeChangeM<(
        Vec<(FigureCanvasKey, FigureCanvasData)>,
        Vec<(FigureControlKey, FigureControlData)>,
    )> {
        self.state
            .presentation
            .update(|presentation| presentation.toggle_pin(trace_id));
        self.update()?;
        let change = self.take_change()?;
        HuskyDevtimeTakeChangeM::Ok((
            change.figure_canvases.opt_new_entries().unwrap_or_default(),
            change.figure_controls.opt_new_entries().unwrap_or_default(),
        ))
    }
}
