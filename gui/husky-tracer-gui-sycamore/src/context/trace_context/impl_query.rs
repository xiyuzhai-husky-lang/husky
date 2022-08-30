use super::*;

impl TraceContext {
    pub(crate) fn is_expanded(&self, trace_id: TraceId) -> bool {
        self.trace_nodes.borrow(file!(), line!())[trace_id.0]
            .expansion
            .cget()
    }

    pub(crate) fn expansion_read_signal(&self, trace_id: TraceId) -> &'static ReadSignal<bool> {
        self.trace_nodes.borrow(file!(), line!())[trace_id.0]
            .expansion
            .read()
    }

    pub(crate) fn is_shown(&self, trace_id: TraceId) -> bool {
        self.trace_nodes.borrow(file!(), line!())[trace_id.0]
            .shown
            .cget()
    }

    pub(crate) fn shown_read_signal(&self, trace_id: TraceId) -> &'static ReadSignal<bool> {
        self.trace_nodes.borrow(file!(), line!())[trace_id.0]
            .shown
            .read()
    }

    pub(crate) fn did_toggle_show(&mut self, trace_id: TraceId) {
        todo!()
    }

    pub(crate) fn did_activate(&self, trace_id: TraceId) {
        self.opt_active_trace_id.set(Some(trace_id))
    }
}
