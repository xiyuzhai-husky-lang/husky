use crate::*;
use husky_visual_protocol::IsVisualProtocol;

pub struct TraceServerCache<VisualProtocol: IsVisualProtocol> {
    cache: TraceCache<VisualProtocol>,
    actions: Vec<TraceAction>,
}
