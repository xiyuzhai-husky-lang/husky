use crate::*;

impl Debugtime {
    pub fn restriction(&self) -> &Restriction {
        &self.state.restriction
    }

    pub fn set_restriction(
        &mut self,
        restriction: Restriction,
    ) -> DebugtimeStageChangeM<(
        Vec<(FigureCanvasKey, FigureCanvasData)>,
        Vec<(FigureControlKey, FigureControlData)>,
        Vec<(TraceStalkKey, TraceStalk)>,
        Vec<(TraceStatsKey, Option<TraceStats>)>,
    )> {
        self.state.restriction.set(restriction)?;
        self.updating();
        self.update_figure_canvases()?;
        self.update_figure_controls()?;
        self.update_trace_stalks();
        self.update_trace_statss()?;
        DebugtimeStageChangeM::Ok((todo!(), todo!(), todo!(), todo!()))
    }
}
