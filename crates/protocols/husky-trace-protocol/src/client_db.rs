use crate::{view::TraceViewData, *};
#[cfg(feature = "mock")]
use husky_visual_protocol::mock::MockVisualProtocol;
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

#[cfg(feature = "mock")]
impl TraceDb<MockVisualProtocol> {
    pub fn new_mock() -> Self {
        Self {
            entries: vec![],
            visual_components: vec![],
        }
    }
}
