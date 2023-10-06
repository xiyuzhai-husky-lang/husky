pub mod error;
pub mod mock;

use self::error::*;
use crate::{message::*, *};
use husky_visual_protocol::mock::MockVisualProtocol;
#[cfg(feature = "mock")]
use husky_visual_protocol::IsVisualProtocol;
use husky_websocket_utils::immediate_websocket_client_connection::{
    ImmediateWebsocketClientConnection, WebsocketClientConnectionError,
};
use tokio::task::JoinHandle;
use tokio_tungstenite::connect_async;

pub struct TraceClient<VisualProtocol: IsVisualProtocol> {
    cache: Option<TraceCache<VisualProtocol>>,
    connection:
        ImmediateWebsocketClientConnection<TraceClientMessage, TraceServerMessage<VisualProtocol>>,
}

impl<VisualProtocol: IsVisualProtocol> TraceClient<VisualProtocol> {
    pub fn new(server_address: impl Into<String>) -> Self {
        Self {
            cache: None,
            connection: ImmediateWebsocketClientConnection::new(server_address.into()),
        }
    }

    pub fn root_trace_ids(&self) -> Option<TraceIdRange> {
        Some(self.cache.as_ref()?.root_trace_ids())
    }

    pub fn connection_error(&self) -> Option<&WebsocketClientConnectionError> {
        self.connection.error()
    }
}

impl<VisualProtocol: IsVisualProtocol> std::ops::Index<TraceIdRange>
    for TraceClient<VisualProtocol>
{
    type Output = [TraceCacheEntry];

    fn index(&self, trace_id_range: TraceIdRange) -> &Self::Output {
        &self.cache.as_ref().unwrap()[trace_id_range]
    }
}
