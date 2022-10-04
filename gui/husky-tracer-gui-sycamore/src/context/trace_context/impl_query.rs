use super::*;

impl DeveloperGuiContext {
    pub(crate) fn is_expanded(&self, trace_id: TraceId) -> bool {
        self.trace_nodes.borrow(file!(), line!())[trace_id.raw()]
            .expansion
            .cget()
    }

    pub(crate) fn expansion_read_signal(&self, trace_id: TraceId) -> &'static ReadSignal<bool> {
        self.trace_nodes.borrow(file!(), line!())[trace_id.raw()]
            .expansion
            .read()
    }

    pub(crate) fn is_shown(&self, trace_id: TraceId) -> bool {
        self.trace_nodes.borrow(file!(), line!())[trace_id.raw()]
            .shown
            .cget()
    }

    pub(crate) fn shown_read_signal(&self, trace_id: TraceId) -> &'static ReadSignal<bool> {
        self.trace_nodes.borrow(file!(), line!())[trace_id.raw()]
            .shown
            .read()
    }

    pub(crate) fn did_toggle_show(&mut self, trace_id: TraceId) {
        todo!()
    }
}
