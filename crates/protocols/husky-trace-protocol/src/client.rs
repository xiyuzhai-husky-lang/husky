pub mod error;
#[cfg(feature = "mock")]
pub mod mock;

use crate::{
    center::action::TraceSynchrotronToggleExpansion, message::*, view::action::TraceViewAction, *,
};
use husky_websocket_utils::imgui_client::{
    ImmediateWebsocketClientConnection, WebsocketClientConnectionError,
};
use notify_change::NotifyChange;
use std::sync::Arc;

pub struct TraceClient<TraceProtocol: IsTraceProtocol, Notifier>
where
    Notifier: NotifyChange,
{
    trace_synchrotron: Option<TraceSynchrotron<TraceProtocol>>,
    connection: ImmediateWebsocketClientConnection<
        TraceRequest<TraceProtocol>,
        TraceResponse<TraceProtocol>,
        Notifier,
    >,
}

impl<TraceProtocol, Notifier> TraceClient<TraceProtocol, Notifier>
where
    TraceProtocol: IsTraceProtocolFull,
    Notifier: NotifyChange,
{
    pub fn new(
        tokio_runtime: Arc<tokio::runtime::Runtime>,
        server_address: impl Into<String>,
        notifier: Notifier,
    ) -> Self {
        Self {
            trace_synchrotron: None,
            connection: ImmediateWebsocketClientConnection::new(
                tokio_runtime,
                server_address.into(),
                notifier,
            ),
        }
    }

    pub fn update(&mut self) {
        let Some(response) = self.connection.try_recv() else {
            return;
        };
        self.process_response(response);
    }

    fn process_response(&mut self, response: TraceResponse<TraceProtocol>) {
        match response {
            TraceResponse::Init { center: cache } => {
                debug_assert!(self.trace_synchrotron.is_none());
                self.trace_synchrotron = Some(cache)
            }
            TraceResponse::TakeTraceSynchrotronAction {
                center_actions: synchrotron_actions,
            } => {
                let Some(ref mut cache) = self.trace_synchrotron else {
                    unreachable!()
                };
                cache.take_actions(synchrotron_actions)
            }
            TraceResponse::Err(e) => panic!("{e}"),
        }
    }

    fn try_send_request(
        &mut self,
        request: TraceRequest<TraceProtocol>,
    ) -> Result<(), WebsocketClientConnectionError> {
        self.connection.try_send_request(request)
    }

    pub fn root_trace_ids(&self) -> Option<&[TraceId]> {
        Some(self.trace_synchrotron.as_ref()?.root_trace_ids())
    }

    pub fn connection_error(&self) -> Option<&WebsocketClientConnectionError> {
        self.connection.error()
    }

    pub fn opt_cache(&self) -> Option<&TraceSynchrotron<TraceProtocol>> {
        self.trace_synchrotron.as_ref()
    }

    #[track_caller]
    fn trace_synchrotron(&self) -> &TraceSynchrotron<TraceProtocol> {
        self.trace_synchrotron.as_ref().unwrap()
    }

    #[track_caller]
    fn cache_mut(&mut self) -> &mut TraceSynchrotron<TraceProtocol> {
        self.trace_synchrotron.as_mut().unwrap()
    }

    pub fn take_view_action(
        &mut self,
        view_action: TraceViewAction<TraceProtocol>,
    ) -> Result<(), WebsocketClientConnectionError> {
        let Some(synchrotron_action) = self.try_resolve_view_action(&view_action) else {
            let synchrotron_actions_len = self.trace_synchrotron().actions_len();
            return self.try_send_request(TraceRequest::TakeViewAction {
                view_action,
                synchrotron_actions_len,
            });
        };
        self.cache_mut().take_action(synchrotron_action.clone());
        match self.try_send_request(TraceRequest::NotifyViewAction {
            view_action,
            center_action: synchrotron_action,
        }) {
            Ok(_) => (),
            Err(e) => match e {
                WebsocketClientConnectionError::SendRequestWhileCreation => todo!(),
                WebsocketClientConnectionError::SendRequestWhileDeserializingRequest => todo!(),
                WebsocketClientConnectionError::SendRequestWhileAwaitingResponse => todo!(),
                WebsocketClientConnectionError::SendRequestWhileSerializingResponse => todo!(),
                WebsocketClientConnectionError::SendRequestWhileResponseNotProcessed => {
                    let response = self.connection.try_recv();
                    todo!("SendRequestWhileResponseNotProcessed, response = {response:?}")
                }
            },
        }
        return Ok(());
    }

    fn try_resolve_view_action(
        &self,
        view_action: &TraceViewAction<TraceProtocol>,
    ) -> Option<TraceSynchrotronAction<TraceProtocol>> {
        match view_action {
            &TraceViewAction::ToggleExpansion { trace_id } => {
                let trace_cache_entry = &self.trace_synchrotron()[trace_id];
                if !trace_cache_entry.expanded() {
                    trace_cache_entry.subtrace_ids()?;
                }
                Some(TraceSynchrotronToggleExpansion::new(trace_id).into())
            }
            TraceViewAction::Marker { _marker } => todo!(),
            TraceViewAction::ToggleExpansion { trace_id: _ } => todo!(),
            &TraceViewAction::ToggleAssociatedTrace {
                trace_id,
                associated_trace_id,
            } => {
                if !self
                    .trace_synchrotron()
                    .is_trace_cached(associated_trace_id)
                {
                    return None;
                }
                Some(TraceSynchrotronAction::ToggleAssociatedTrace {
                    trace_id,
                    associated_trace_id,
                })
            }
        }
    }
}
