pub mod error;
pub mod mock;

use self::error::*;
use crate::{message::*, *};
use husky_visual_protocol::mock::MockVisualProtocol;
#[cfg(feature = "mock")]
use husky_visual_protocol::IsVisualProtocol;
use husky_websocket_utils::immediate_client::{
    ImmediateWebsocketClientConnection, WebsocketClientConnectionError,
};
use tokio::task::JoinHandle;
use tokio_tungstenite::connect_async;

pub struct TraceClient<
    VisualProtocol: IsVisualProtocol,
    ResponseNotifier: notify_change::NotifyEvent,
> {
    cache: Option<TraceCache<VisualProtocol>>,
    connection: ImmediateWebsocketClientConnection<
        TraceRequest,
        TraceResponse<VisualProtocol>,
        ResponseNotifier,
    >,
}

impl<VisualProtocol, TraceServerMessageArrivalNotifier>
    TraceClient<VisualProtocol, TraceServerMessageArrivalNotifier>
where
    VisualProtocol: IsVisualProtocol,
    TraceServerMessageArrivalNotifier: notify_change::NotifyEvent,
{
    pub fn new(
        server_address: impl Into<String>,
        notifier: TraceServerMessageArrivalNotifier,
    ) -> Self {
        Self {
            cache: None,
            connection: ImmediateWebsocketClientConnection::new(
                server_address.into(),
                TraceRequest::Init,
                notifier,
            ),
        }
    }

    pub fn root_trace_ids(&self) -> Option<TraceIdRange> {
        Some(self.cache.as_ref()?.root_trace_ids())
    }

    pub fn connection_error(&self) -> Option<&WebsocketClientConnectionError> {
        self.connection.error()
    }
}

impl<VisualProtocol, TraceServerMessageArrivalNotifier> std::ops::Index<TraceIdRange>
    for TraceClient<VisualProtocol, TraceServerMessageArrivalNotifier>
where
    VisualProtocol: IsVisualProtocol,
    TraceServerMessageArrivalNotifier: notify_change::NotifyEvent,
{
    type Output = [TraceCacheEntry];

    fn index(&self, trace_id_range: TraceIdRange) -> &Self::Output {
        &self.cache.as_ref().unwrap()[trace_id_range]
    }
}
