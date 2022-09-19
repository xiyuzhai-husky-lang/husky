use crate::*;

impl Debugtime {
    pub fn restriction(&self) -> &Restriction {
        &self.state.restriction
    }

    pub fn set_restriction(
        &mut self,
        restriction: Restriction,
    ) -> DebugtimeUpdatedM<(
        Vec<(FigureCanvasKey, FigureCanvasData)>,
        Vec<(FigureControlKey, FigureControlData)>,
        Vec<(TraceStalkKey, TraceStalk)>,
        Vec<(TraceStatsKey, Option<TraceStats>)>,
    )> {
        self.state.restriction = restriction;
        DebugtimeUpdatedM::Ok((
            self.update_figure_canvases()?,
            self.update_figure_controls()?,
            self.update_trace_stalks(),
            self.update_trace_statss()?,
        ))
    }
}

// let (opt_figure_canvas_data, opt_figure_control_data) = if let Some(active_trace_id) =
// self.debugtime.opt_active_trace_id()
// {
// let opt_figure_canvas_data = if needs_figure_canvases {
//     match self.debugtime.figure_canvas(active_trace_id) {
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
//     Some(self.debugtime.figure_control(active_trace_id))
// } else {
//     None
// };
// (opt_figure_canvas_data, opt_figure_control_data)
// } else {
// (None, None)
// };
