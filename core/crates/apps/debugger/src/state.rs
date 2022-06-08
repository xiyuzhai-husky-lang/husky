use json_map::JsonListMap;

use crate::*;

#[derive(Default)]
pub struct DebuggerState {
    pub(crate) active_trace_id: Option<TraceId>,
    pub(crate) subtraces_map: JsonListMap<(TraceId, Option<usize>), Vec<TraceId>>,
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
