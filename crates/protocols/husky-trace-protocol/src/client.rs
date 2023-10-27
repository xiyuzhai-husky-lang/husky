pub mod error;
pub mod mock;

use std::sync::Arc;

use self::error::*;
use crate::{message::*, view::action::TraceViewAction, *};
#[cfg(feature = "mock")]
use husky_visual_protocol::IsVisualProtocol;
use husky_visual_protocol::{mock::MockVisualProtocol, IsVisualComponent};
use husky_websocket_utils::imgui_client::{
    ImmediateWebsocketClientConnection, WebsocketClientConnectionError,
};
use tokio::task::JoinHandle;
use tokio_tungstenite::connect_async;

pub struct TraceClient<VisualComponent: IsVisualComponent> {
    cache: Option<TraceCache<VisualComponent>>,
    connection: ImmediateWebsocketClientConnection<TraceRequest, TraceResponse<VisualComponent>>,
}

impl<VisualComponent: IsVisualComponent> TraceClient<VisualComponent>
where
    VisualComponent: IsVisualComponent,
{
    pub fn new(
        tokio_runtime: Arc<tokio::runtime::Runtime>,
        server_address: impl Into<String>,
    ) -> Self {
        Self {
            cache: None,
            connection: ImmediateWebsocketClientConnection::new(
                tokio_runtime,
                server_address.into(),
            ),
        }
    }

    pub fn update(&mut self) {
        let Some(response) = self.connection.try_recv() else {
            return;
        };
        self.accept_response(response);
    }

    fn accept_response(&mut self, response: TraceResponse<VisualComponent>) {
        match response {
            TraceResponse::Init { cache } => {
                debug_assert!(self.cache.is_none());
                self.cache = Some(cache)
            }
        }
    }

    pub fn root_trace_ids(&self) -> Option<&[TraceId]> {
        Some(self.cache.as_ref()?.root_trace_ids())
    }

    pub fn connection_error(&self) -> Option<&WebsocketClientConnectionError> {
        self.connection.error()
    }

    pub fn cache(&self) -> Option<&TraceCache<VisualComponent>> {
        self.cache.as_ref()
    }

    pub fn take_action(&mut self, action: TraceViewAction<VisualComponent>) {
        match action {
            TraceViewAction::ToggleExpansion { trace_id } => todo!(),
            TraceViewAction::Marker { _marker } => todo!(),
        }
    }
}
