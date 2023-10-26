#[cfg(feature = "client")]
use husky_websocket_utils::imgui_client::NeedResponse;

use super::*;

/// message sent from trace client to trace server
#[derive(Debug, Serialize, Deserialize)]
pub enum TraceRequest {
    Init,
}

impl Default for TraceRequest {
    fn default() -> Self {
        TraceRequest::Init
    }
}

#[cfg(feature = "client")]
impl NeedResponse for TraceRequest {
    fn need_response(&self) -> bool {
        match self {
            TraceRequest::Init => true,
        }
    }
}
