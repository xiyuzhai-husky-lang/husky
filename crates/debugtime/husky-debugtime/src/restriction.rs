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
        HuskyDebugtimeTakeChangeM::Ok((todo!(), todo!(), todo!(), todo!()))
    }
}
