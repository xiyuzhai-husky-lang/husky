use husky_trace_time::HuskyTraceTime;

pub struct HuskyDebuggerInternal {
    pub(crate) tracetime: HuskyTraceTime,
    pub(crate) next_request_id: usize,
}
