#[cfg(feature = "client")]
use husky_websocket_utils::imgui_client::NeedResponse;

use crate::view::action::TraceViewAction;

use super::*;

/// message sent from trace client to trace server
#[derive(Debug, Serialize, Deserialize)]
pub enum TraceRequest<TraceProtocol: IsTraceProtocol> {
    Init {
        trace_protocol_type_name: String,
    },
    /// view action are not handled on client side,
    /// ask the server to handle it and return cache actions
    TakeViewAction {
        view_action: TraceViewAction<TraceProtocol>,
        trace_synchrotron_status: TraceSynchrotronStatus,
    },
    /// view action already handled on client side,
    /// ask the server to do the same
    NotifyViewAction {
        view_action: TraceViewAction<TraceProtocol>,
        center_action: TraceSynchrotronAction<TraceProtocol>,
    },
}

impl<TraceProtocol> Default for TraceRequest<TraceProtocol>
where
    TraceProtocol: IsTraceProtocol + Default,
{
    fn default() -> Self {
        TraceRequest::Init {
            trace_protocol_type_name: std::any::type_name::<TraceProtocol>().to_string(),
        }
    }
}

#[cfg(feature = "client")]
impl<TraceProtocol> NeedResponse for TraceRequest<TraceProtocol>
where
    TraceProtocol: IsTraceProtocol + Default,
{
    fn need_response(&self) -> bool {
        match self {
            TraceRequest::Init { .. } => true,
            TraceRequest::TakeViewAction { .. } => true,
            TraceRequest::NotifyViewAction { .. } => false,
        }
    }
}
