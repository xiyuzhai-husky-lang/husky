use crate::*;

impl HuskyDevtime {
    pub fn presentation(&self) -> &Presentation {
        &self.state.presentation
    }

    pub fn set_restriction(
        &mut self,
        restriction: Presentation,
    ) -> HuskyDevtimeTakeChangeM<(
        Vec<(FigureCanvasKey, FigureCanvasData)>,
        Vec<(FigureControlKey, FigureControlData)>,
        Vec<(TraceStalkKey, TraceStalk)>,
        Vec<(TraceStatsKey, Option<TraceStats>)>,
    )> {
        self.state.presentation.set(restriction)?;
        self.update()?;
        let change = self.take_change()?;
        HuskyDevtimeTakeChangeM::Ok((
            change.figure_canvases.opt_new_entries().unwrap_or_default(),
            change.figure_controls.opt_new_entries().unwrap_or_default(),
            change.trace_stalks.opt_new_entries().unwrap_or_default(),
            change.trace_statss.opt_new_entries().unwrap_or_default(),
        ))
    }
}
