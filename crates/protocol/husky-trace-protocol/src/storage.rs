use crate::{view::TraceViewData, *};
use husky_task::{visual::VisualComponent, IsTask};

pub struct TraceStorage<Task: IsTask> {
    entries: TraceStorageProtocolEntry,
    visual_elements: Vec<VisualComponent<Task>>,
}

pub struct TraceStorageProtocolEntry {
    data: TraceViewData,
}

pub struct TraceStorageRef<'a> {
    entries: &'a [TraceStorageProtocolEntry],
}
