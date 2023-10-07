use crate::*;
use husky_visual_protocol::IsVisualProtocol;

pub struct TraceServer<VisualProtocol: IsVisualProtocol> {
    cache: TraceCache<VisualProtocol>,
    actions: Vec<TraceAction>,
}

impl<VisualProtocol: IsVisualProtocol> Default for TraceServer<VisualProtocol> {
    fn default() -> Self {
        Self {
            cache: Default::default(),
            actions: Default::default(),
        }
    }
}
