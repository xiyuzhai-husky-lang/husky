use super::*;
use web_sys::{Event, HtmlDialogElement, HtmlInputElement, KeyboardEvent};

impl DebuggerContext {
    pub(super) fn needs_figure_canvas_data(
        &'static self,
        opt_active_trace_id: Option<TraceId>,
        restriction: &Restriction,
    ) -> bool {
        let active_trace_id = if let Some(active_trace_id) = opt_active_trace_id {
            active_trace_id
        } else {
            return false;
        };
        let trace = self.trace_context.trace_data(active_trace_id);
        // let restriction = opt_new_restriction.unwrap_or(&self.restriction_context.restriction.get());
        let key = self
            .figure_context
            .new_figure_canvas_key(trace, restriction);
        !self
            .figure_context
            .figure_canvases
            .borrow(file!(), line!())
            .contains_key(&key)
    }

    pub(super) fn needs_figure_control_data(
        &self,
        opt_active_trace_id: Option<TraceId>,
        restriction: &Restriction,
    ) -> bool {
        let active_trace_id = if let Some(active_trace_id) = opt_active_trace_id {
            active_trace_id
        } else {
            return false;
        };
        let trace = self.trace_context.trace_data(active_trace_id);
        let key = FigureControlKey::from_trace_data(trace, restriction);
        !self
            .figure_context
            .figure_controls
            .borrow(file!(), line!())
            .contains_key(&key)
    }

    pub(super) fn needs_stalk(
        &self,
        opt_active_trace_id: Option<TraceId>,
        restriction: &Restriction,
    ) -> bool {
        // let active_trace_id = if let Some(active_trace_id) = opt_active_trace_id {
        //     active_trace_id
        // } else {
        //     return false;
        // };
        // let trace = self.trace_context.trace(active_trace_id);
        let sample_id = match restriction.opt_sample_id() {
            Some(sample_id) => sample_id,
            None => return false,
        };
        !self
            .trace_context
            .for_all_expanded_traces(Some(sample_id), |trace_data| {
                let key = TraceStalkKey::from_trace_data(sample_id, trace_data);
                self.trace_context
                    .trace_stalks
                    .borrow(file!(), line!())
                    .contains_key(&key)
            })
    }
}
