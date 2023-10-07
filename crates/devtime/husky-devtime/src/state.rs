mod old;

pub use old::*;

use crate::*;
use husky_trace_protocol::server::TraceServer;
use husky_visual_protocol::IsVisualProtocol;

pub type DevtimeState111 = TraceWorld<TraceNode>;

pub type DevtimeOldState = ServerTraceOldState<TraceNode>;

pub type DevtimeStateChange = ServerTraceStateChange;

#[derive(Default)]
pub struct DevtimeState<VisualProtocol: IsVisualProtocol> {
    server: TraceServer<VisualProtocol>,
}
