use crate::*;
use husky_trace_protocol::server::TraceServer;
use husky_visual_protocol::IsVisualProtocol;

#[derive()]
pub struct DevtimeState<VisualProtocol: IsVisualProtocol> {
    server: TraceServer<VisualProtocol>,
}

impl<VisualProtocol: IsVisualProtocol> Default for DevtimeState<VisualProtocol> {
    fn default() -> Self {
        Self {
            server: Default::default(),
        }
    }
}
