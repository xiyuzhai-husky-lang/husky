use crate::*;

#[derive(Default)]
pub struct DebuggerState {
    pub(crate) active_trace_id: Option<TraceId>,
}
