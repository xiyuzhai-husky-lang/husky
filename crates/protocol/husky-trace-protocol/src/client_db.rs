use crate::{view::TraceViewData, *};
use husky_task::{visual::VisualComponent, IsTask};

/// synced across server and client
pub struct TraceDb<Task: IsTask> {
    entries: Vec<TraceViewEntry>,
    visual_components: Vec<VisualComponent<Task>>,
}

pub struct TraceViewEntry {
    data: TraceViewData,
}

pub struct TraceStorageRef<'a> {
    entries: &'a [TraceViewEntry],
}
