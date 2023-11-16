#[cfg(feature = "client")]
use husky_websocket_utils::imgui_client::NeedResponse;

use crate::view::action::TraceViewAction;

use super::*;

/// message sent from trace client to trace server
#[derive(Debug, Serialize, Deserialize)]
pub enum TraceRequest<TraceProtocol> {
    Init {
        trace_protocol: TraceProtocol,
    },
    /// view action are not handled on client side,
    /// ask the server to handle it and return cache actions
    TakeViewAction {
        view_action: TraceViewAction<TraceProtocol>,
        cache_actions_len: usize,
    },
    /// view action already handled on client side,
    /// ask the server to do the same
    NotifyViewAction {
        view_action: TraceViewAction<TraceProtocol>,
        cache_action: TraceCacheAction<TraceProtocol>,
    },
}

impl<TraceProtocol> Default for TraceRequest<TraceProtocol>
where
    TraceProtocol: Default,
{
    fn default() -> Self {
        TraceRequest::Init {
            trace_protocol: Default::default(),
        }
    }
}

#[cfg(feature = "client")]
impl<VisualComponent> NeedResponse for TraceRequest<VisualComponent> {
    fn need_response(&self) -> bool {
        match self {
            TraceRequest::Init { .. } => true,
            TraceRequest::TakeViewAction { .. } => true,
            TraceRequest::NotifyViewAction { .. } => false,
        }
    }
}
