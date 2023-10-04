use husky_trace_protocol::{storage::TraceStorageRef, view::action::TraceViewAction};

pub struct TraceView<'a> {
    storage_ref: TraceStorageRef<'a>,
    action: Option<TraceViewAction>,
}
