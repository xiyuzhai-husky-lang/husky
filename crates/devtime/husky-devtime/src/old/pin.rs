use super::*;

impl<Task: IsTask> Devtime<Task> {
    pub fn toggle_pin(&mut self, trace_id: TraceId) -> DevtimeStateChange {
        todo!()
        // self.state
        //     .update_presentation(|presentation| presentation.toggle_pin(trace_id));
        // self.update()?;
        // self.take_change()
    }
}
