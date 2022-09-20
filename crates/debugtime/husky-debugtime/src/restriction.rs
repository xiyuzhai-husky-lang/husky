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
        self.update()?;
        DebugtimeStageChangeM::Ok((todo!(), todo!(), todo!(), todo!()))
    }
}
