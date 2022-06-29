use crate::*;
use husky_trace_time::HuskyTraceTime;

pub struct HuskyDebuggerInternal {
    pub(crate) trace_time: HuskyTraceTime,
    pub(crate) config: DebuggerConfig,
    pub(crate) next_request_id: usize,
}
