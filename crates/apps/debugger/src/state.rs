use json_map::JsonMap;

use crate::{impl_figure::FigureControlProps, *};

#[derive(Default)]
pub struct DebuggerState {
    pub(crate) active_trace_id: Option<TraceId>,
    pub(crate) subtraces_map: JsonMap<(TraceId, Option<usize>), Vec<TraceId>>,
    pub(crate) figure_control: JsonMap<TraceId, FigureControlProps>,
}

impl DebuggerState {
    pub fn set_subtraces(
        &mut self,
        parent: &Trace,
        effective_opt_input_id: Option<usize>,
        // effective_opt_input_id_for_subtraces: Option<usize>,
        subtraces: &[Arc<Trace>],
    ) {
        assert!(self
            .subtraces_map
            .insert(
                (parent.id(), effective_opt_input_id),
                subtraces.iter().map(|trace| trace.id()).collect()
            )
            .is_none())
    }
}
