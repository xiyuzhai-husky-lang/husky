use std::marker::PhantomData;

use crate::*;
use husky_trace_protocol::server::TraceServer;
use husky_visual_protocol::IsVisualProtocol;

#[derive()]
pub struct DevtimeState {
    // server: TraceServer<VisualProtocol>,
}

impl Default for DevtimeState {
    fn default() -> Self {
        Self {}
    }
}
