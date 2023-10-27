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
    connection: ImmediateWebsocketClientConnection<
        TraceRequest<VisualComponent>,
        TraceResponse<VisualComponent>,
    >,
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
        self.process_response(response);
    }

    fn process_response(&mut self, response: TraceResponse<VisualComponent>) {
        match response {
            TraceResponse::Init { cache } => {
                debug_assert!(self.cache.is_none());
                self.cache = Some(cache)
            }
        }
    }

    fn try_send_request(
        &mut self,
        request: TraceRequest<VisualComponent>,
    ) -> Result<(), WebsocketClientConnectionError> {
        self.connection.try_send_request(request)
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

    pub fn take_view_action(
        &mut self,
        view_action: TraceViewAction<VisualComponent>,
    ) -> Result<(), WebsocketClientConnectionError> {
        let Some(ref mut cache) = self.cache else {
            unreachable!()
        };
        match view_action.try_resolve_at_client_side(cache) {
            Some(cache_action) => {
                cache.take_action(cache_action.clone());

                self.try_send_request(TraceRequest::NotifyViewAction {
                    view_action,
                    cache_action,
                })
                .expect("should be okay");
                Ok(())
            }
            None => {
                let cache_actions_len = cache.actions_len();
                self.try_send_request(TraceRequest::TakeViewAction {
                    view_action,
                    cache_actions_len,
                })
            }
        }
    }
}
