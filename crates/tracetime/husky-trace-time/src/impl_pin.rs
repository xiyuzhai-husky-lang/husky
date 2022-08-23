use super::*;

impl HuskyTraceTime {
    pub fn toggle_pin(&mut self, trace_id: TraceId) {
        self.pins.toggle(trace_id)
    }
}
