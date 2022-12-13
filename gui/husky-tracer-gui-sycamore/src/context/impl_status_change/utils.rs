use super::*;
use web_sys::{Event, HtmlDialogElement, HtmlInputElement, KeyboardEvent};

impl DeveloperGuiContext {
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
        for pin in presentation.pins().iter() {
            if self.needs_figure_canvas(*pin, presentation) {
                return true;
            }
        }
        false
    }

    fn needs_figure_canvas(&self, trace_id: TraceId, presentation: &Presentation) -> bool {
        let trace = self.trace_data(trace_id);
        if let Some(key) = SpecificFigureCanvasKey::from_trace_data(trace, presentation) {
            if !self
                .specific_figure_canvases
                .borrow(file!(), line!())
                .contains_key(&key)
            {
                return true;
            }
        }
        if let Some(key) = GenericFigureCanvasKey::from_trace_data(trace, presentation) {
            if !self
                .generic_figure_canvases
                .borrow(file!(), line!())
                .contains_key(&key)
            {
                return true;
            }
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
        for pin in presentation.pins().iter() {
            if self.needs_figure_control(*pin, presentation) {
                return true;
            }
        }
        false
    }

    fn needs_figure_control(&self, trace_id: TraceId, presentation: &Presentation) -> bool {
        let trace_data = self.trace_data(trace_id);
        let key = FigureControlKey::from_trace_data(trace_data, presentation);
        !self
            .figure_controls
            .borrow(file!(), line!())
            .contains_key(&key)
    }

    pub(super) fn needs_stalks(&self, presentation: &Presentation) -> bool {
        let sample_id = match presentation.opt_sample_id() {
            Some(sample_id) => sample_id,
            None => return false,
        };
        !self.for_all_immediate_traces(Some(sample_id), |trace_data| {
            let key = TraceStalkKey::from_trace_data(sample_id, trace_data);
            self.trace_stalks
                .borrow(file!(), line!())
                .contains_key(&key)
        })
    }

    pub(super) fn needs_statss(&self, new_presentation: &Presentation) -> bool {
        !self.for_all_immediate_traces(new_presentation.opt_sample_id(), |trace_data| {
            let key = TraceStatsKey {
                trace_id: trace_data.id,
                partitions: new_presentation.partitions().clone(),
            };
            self.trace_statss
                .borrow(file!(), line!())
                .contains_key(&key)
        })
    }
}
