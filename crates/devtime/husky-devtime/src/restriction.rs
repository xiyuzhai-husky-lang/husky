use crate::*;

impl HuskyDebugtime {
    pub fn restriction(&self) -> &Restriction {
        &self.state.restriction
    }

    pub fn set_restriction(
        &mut self,
        restriction: Restriction,
    ) -> HuskyDebugtimeTakeChangeM<(
        Vec<(FigureCanvasKey, FigureCanvasData)>,
        Vec<(FigureControlKey, FigureControlData)>,
        Vec<(TraceStalkKey, TraceStalk)>,
        Vec<(TraceStatsKey, Option<TraceStats>)>,
    )> {
        self.state.restriction.set(restriction)?;
        self.update()?;
        let change = self.take_change()?;
        HuskyDebugtimeTakeChangeM::Ok((
            change.figure_canvases.opt_new_entries().unwrap_or_default(),
            change.figure_controls.opt_new_entries().unwrap_or_default(),
            change.trace_stalks.opt_new_entries().unwrap_or_default(),
            change.trace_statss.opt_new_entries().unwrap_or_default(),
        ))
    }
}
