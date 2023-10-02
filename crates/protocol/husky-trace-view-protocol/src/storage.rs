use crate::*;

#[derive(Serialize, Deserialize)]
pub struct TraceViewStorage {
    entries: Vec<TraceViewEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct TraceViewEntry {
    trace_view_data: TraceViewData,
}

#[derive(Serialize, Deserialize)]
pub enum TraceViewAction {
    AllocTrace { id: TraceId, entry: TraceViewEntry },
    SetExpansion { id: TraceId },
}

#[derive(Serialize, Deserialize)]
pub struct TraceId(u32);
