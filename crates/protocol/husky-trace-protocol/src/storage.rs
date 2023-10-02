use crate::*;
use husky_task::{visual::VisualElement, IsTask};

pub struct TraceBuffer<Task: IsTask> {
    entries: TraceStorageProtocolEntry,
    visual_elements: Vec<VisualElement<Task>>,
}

pub struct TraceStorageProtocolEntry {
    data: TraceViewData,
}
