use super::*;

impl DebuggerContext {
    fn enter_signal(&self, trace_id: TraceId) -> &'static Signal<bool> {
        self.trace_context.trace_nodes.borrow(file!(), line!())[trace_id.0].enter
    }

    pub(super) fn toggle_enter(&'static self, trace_id: TraceId) {
        todo!()
    }
}
