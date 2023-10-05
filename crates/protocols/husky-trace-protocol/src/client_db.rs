use crate::{view::TraceViewData, *};
use husky_visual_protocol::{IsVisualProtocol, VisualComponent};

/// synced across server and client
pub struct TraceDb<VisualProtocol: IsVisualProtocol> {
    entries: Vec<TraceViewEntry>,
    visual_components: Vec<VisualComponent<VisualProtocol>>,
}

pub struct TraceViewEntry {
    data: TraceViewData,
}

pub struct TraceStorageRef<'a> {
    entries: &'a [TraceViewEntry],
}
