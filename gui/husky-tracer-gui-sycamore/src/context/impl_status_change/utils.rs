use super::*;
use web_sys::{Event, HtmlDialogElement, HtmlInputElement, KeyboardEvent};

impl DebuggerContext {
    pub(super) fn needs_figure_canvases(
        &self,
        opt_active_trace_id: Option<TraceId>,
        presentation: &Presentation,
    ) -> bool {
        if let Some(active_trace_id) = opt_active_trace_id {
            if self.needs_figure_canvas(active_trace_id, presentation) {
                return true;
            }
        };
        for pin in self.pins.get().iter() {
            if self.needs_figure_canvas(*pin, presentation) {
                return true;
            }
        }
        false
    }

    fn needs_figure_canvas(&self, trace_id: TraceId, restriction: &Presentation) -> bool {
        let trace = self.trace_context.trace_data(trace_id);
        let key = self.new_figure_canvas_key(trace, restriction, true);
        if !self
            .figure_canvases
            .borrow(file!(), line!())
            .contains_key(&key)
        {
            return true;
        }
        let key = self.new_figure_canvas_key(trace, restriction, false);
        if !self
            .figure_canvases
            .borrow(file!(), line!())
            .contains_key(&key)
        {
            return true;
        }
        false
    }

    pub(super) fn needs_figure_controls(
        &self,
        opt_active_trace_id: Option<TraceId>,
        presentation: &Presentation,
    ) -> bool {
        if let Some(active_trace_id) = opt_active_trace_id {
            if self.needs_figure_control(active_trace_id, presentation) {
                return true;
            }
        }
        for pin in self.pins.get().iter() {
            if self.needs_figure_control(*pin, presentation) {
                return true;
            }
        }
        false
    }

    fn needs_figure_control(&self, trace_id: TraceId, restriction: &Presentation) -> bool {
        let trace_data = self.trace_context.trace_data(trace_id);
        let key = FigureControlKey::from_trace_data(trace_data, restriction);
        !self
            .figure_controls
            .borrow(file!(), line!())
            .contains_key(&key)
    }

    #[cfg(feature = "verify_consistency")]
    pub(super) fn new_stalk_keys(&self, restriction: &Presentation) -> Vec<TraceStalkKey> {
        let sample_id = match restriction.opt_sample_id() {
            Some(sample_id) => sample_id,
            None => return vec![],
        };
        self.trace_context
            .filter_immediate_traces(Some(sample_id), |trace_data| {
                let key = TraceStalkKey::from_trace_data(sample_id, trace_data);
                if !self
                    .trace_context
                    .trace_stalks
                    .borrow(file!(), line!())
                    .contains_key(&key)
                {
                    Some(key)
                } else {
                    None
                }
            })
    }

    #[cfg(not(feature = "verify_consistency"))]
    pub(super) fn needs_stalks(&self, restriction: &Presentation) -> bool {
        let sample_id = match restriction.opt_sample_id() {
            Some(sample_id) => sample_id,
            None => return false,
        };
        !self
            .trace_context
            .for_all_immediate_traces(Some(sample_id), |trace_data| {
                let key = TraceStalkKey::from_trace_data(sample_id, trace_data);
                self.trace_context
                    .trace_stalks
                    .borrow(file!(), line!())
                    .contains_key(&key)
            })
    }

    #[cfg(feature = "verify_consistency")]
    pub(super) fn new_stats_keys(&self, new_restriction: &Presentation) -> Vec<TraceStatsKey> {
        self.trace_context
            .filter_immediate_traces(new_restriction.opt_sample_id(), |trace_data| {
                let key = TraceStatsKey {
                    trace_id: trace_data.id,
                    partitions: new_restriction.partitions().clone(),
                };
                if !self
                    .trace_context
                    .trace_statss
                    .borrow(file!(), line!())
                    .contains_key(&key)
                {
                    Some(key)
                } else {
                    None
                }
            })
    }

    #[cfg(not(feature = "verify_consistency"))]
    pub(super) fn needs_statss(&self, new_restriction: &Presentation) -> bool {
        !self.trace_context.for_all_immediate_traces(
            new_restriction.opt_sample_id(),
            |trace_data| {
                let key = TraceStatsKey {
                    trace_id: trace_data.id,
                    partitions: new_restriction.partitions().clone(),
                };
                self.trace_context
                    .trace_statss
                    .borrow(file!(), line!())
                    .contains_key(&key)
            },
        )
    }
}
