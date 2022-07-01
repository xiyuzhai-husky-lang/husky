use super::*;
use web_sys::{Event, HtmlDialogElement, HtmlInputElement, KeyboardEvent};

impl DebuggerContext {
    pub(super) fn needs_figure_canvas_data(
        &'static self,
        opt_active_trace_id: Option<TraceId>,
        attention: &Attention,
    ) -> bool {
        let active_trace_id = if let Some(active_trace_id) = opt_active_trace_id {
            active_trace_id
        } else {
            return false;
        };
        let trace = self.trace_context.trace(active_trace_id);
        // let attention = opt_new_attention.unwrap_or(&self.attention_context.attention.get());
        let key = FigureCanvasKey::new(trace.kind, trace.id, attention);
        !self
            .figure_context
            .figure_canvases
            .borrow(file!(), line!())
            .contains_key(&key)
    }

    pub(super) fn needs_figure_control_data(
        &self,
        opt_active_trace_id: Option<TraceId>,
        attention: &Attention,
    ) -> bool {
        let active_trace_id = if let Some(active_trace_id) = opt_active_trace_id {
            active_trace_id
        } else {
            return false;
        };
        let trace = self.trace_context.trace(active_trace_id);
        let key = FigureControlKey::from_trace_data(trace, attention);
        !self
            .figure_context
            .figure_controls
            .borrow(file!(), line!())
            .contains_key(&key)
    }

    pub(super) fn needs_stalk(
        &self,
        opt_active_trace_id: Option<TraceId>,
        attention: &Attention,
    ) -> bool {
        let active_trace_id = if let Some(active_trace_id) = opt_active_trace_id {
            active_trace_id
        } else {
            return false;
        };
        let trace = self.trace_context.trace(active_trace_id);
        let sample_id = match attention.opt_sample_id() {
            Some(sample_id) => sample_id,
            None => return false,
        };
        let key = TraceStalkKey::from_trace_data(sample_id, trace);
        !self
            .trace_context
            .trace_stalks
            .borrow(file!(), line!())
            .contains_key(&key)
    }
}
