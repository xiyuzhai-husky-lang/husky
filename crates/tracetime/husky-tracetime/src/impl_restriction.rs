use crate::*;

impl HuskyTracetime {
    pub fn restriction(&self) -> &Restriction {
        &self.restriction
    }

    pub fn set_restriction(
        &mut self,
        restriction: Restriction,
    ) -> __VMResult<(
        Vec<(FigureCanvasKey, FigureCanvasData)>,
        Vec<(FigureControlKey, FigureControlData)>,
        Vec<(TraceStalkKey, TraceStalk)>,
        Vec<(TraceStatsKey, Option<TraceStats>)>,
    )> {
        self.restriction = restriction;
        Ok((
            self.update_figure_canvases()?,
            self.update_figure_controls()?,
            self.update_trace_stalks(),
            self.update_trace_statss()?,
        ))
    }
}

// let (opt_figure_canvas_data, opt_figure_control_data) = if let Some(active_trace_id) =
// self.tracetime.opt_active_trace_id()
// {
// let opt_figure_canvas_data = if needs_figure_canvases {
//     match self.tracetime.figure_canvas(active_trace_id) {
//         Ok(figure_canvas) => Some(figure_canvas),
//         Err((sample_id, error)) => {
//             return Some(HuskyTracerServerMessageVariant::SetRestrictionWithError {
//                 sample_id,
//                 error: format!("{:?}", error),
//             })
//         }
//     }
// } else {
//     None
// };
// let opt_figure_control_data = if needs_figure_controls {
//     Some(self.tracetime.figure_control(active_trace_id))
// } else {
//     None
// };
// (opt_figure_canvas_data, opt_figure_control_data)
// } else {
// (None, None)
// };
