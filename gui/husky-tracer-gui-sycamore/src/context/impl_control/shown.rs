use super::*;
use web_sys::{Event, HtmlDialogElement, HtmlInputElement, KeyboardEvent};

impl DebuggerContext {
    pub(crate) fn toggle_shown(&'static self, trace_id: TraceId) {
        let shown_signal = self.trace_context.shown_signal(trace_id);
        shown_signal.set(!shown_signal.cget());
        self.ws
            .send_message(HuskyTracerGuiMessageVariant::ToggleShow { trace_id }, None)
    }
}
