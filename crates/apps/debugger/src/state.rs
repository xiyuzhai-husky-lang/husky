use json_map::JsonMap;

use crate::*;

#[derive(Default)]
pub struct DebuggerState {
    pub(crate) active_trace_id: Option<TraceId>,
    pub(crate) subtraces_map: JsonMap<(TraceId, Option<usize>), Vec<TraceId>>,
}

impl DebuggerState {
    pub fn set_subtraces(
        &mut self,
        parent_id: TraceId,
        effective_opt_input_id_for_subtraces: Option<usize>,
        subtraces: &[Arc<Trace>],
    ) {
        assert!(self
            .subtraces_map
            .insert(
                (parent_id, effective_opt_input_id_for_subtraces),
                subtraces.iter().map(|trace| trace.id()).collect()
            )
            .is_none())
    }
}
