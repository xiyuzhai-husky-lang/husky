use super::*;

impl HuskyDevtime {
    pub fn toggle_pin(
        &mut self,
        trace_id: TraceId,
    ) -> HuskyDevtimeTakeChangeM<HuskyDevtimeStateChange> {
        self.state
            .update_presentation(|presentation| presentation.toggle_pin(trace_id));
        self.update()?;
        self.take_change()
    }
}
