pub mod error;
#[cfg(feature = "mock")]
pub mod mock;
#[cfg(feature = "test_utils")]
pub mod test_utils;

use crate::{
    message::*, synchrotron::action::TraceSynchrotronToggleExpansion,
    view::action::TraceViewAction, *,
};
use husky_devsoul_interface::pedestal::IsPedestal;
use husky_websocket_utils::imgui_client::{
    ImmediateWebsocketClientConnection, WebsocketClientConnectionError,
};
use notify_change::NotifyChange;
use std::sync::Arc;

use self::{accompany::AccompanyingTraceIdsExceptFollowed, caryatid::TraceCaryatidUiBuffer};

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

    pub fn update(
        &mut self,
        pedestal_ui_buffer: &mut Option<TraceCaryatidUiBuffer<TraceProtocol>>,
    ) {
        let Some(response) = self.connection.try_recv() else {
            return;
        };
        self.process_response(response, pedestal_ui_buffer);
    }

    fn process_response(
        &mut self,
        response: TraceResponse<TraceProtocol>,
        pedestal_ui_buffer: &mut Option<TraceCaryatidUiBuffer<TraceProtocol>>,
    ) {
        match response {
            TraceResponse::Init { trace_synchrotron } => {
                debug_assert!(self.trace_synchrotron.is_none());
                *pedestal_ui_buffer = todo!();
                // Some(trace_synchrotron.caryatid().init_ui_buffer());
                self.trace_synchrotron = Some(trace_synchrotron)
            }
            TraceResponse::TakeTraceSynchrotronActionsDiff {
                trace_synchrotron_actions_diff,
            } => {
                let Some(ref mut trace_synchrotron) = self.trace_synchrotron else {
                    unreachable!()
                };
                trace_synchrotron.take_actions_diff(trace_synchrotron_actions_diff);
                // pedestal_ui_buffer
                //     .as_mut()
                //     .unwrap()
                //     .update(trace_synchrotron.pedestal());
                todo!()
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

    pub fn connection_error(&self) -> Option<&WebsocketClientConnectionError> {
        self.connection.error()
    }

    pub fn opt_trace_synchrotron(&self) -> Option<&TraceSynchrotron<TraceProtocol>> {
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
            let trace_synchrotron_status = self.trace_synchrotron().status();
            return self.try_send_request(TraceRequest::TakeViewAction {
                view_action,
                trace_synchrotron_status,
            });
        };
        self.cache_mut().take_action(synchrotron_action.clone());
        match self.try_send_request(TraceRequest::NotifyViewAction {
            view_action,
            trace_synchrotron_action: synchrotron_action,
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
        match *view_action {
            TraceViewAction::ToggleExpansion { trace_id } => {
                let trace_cache_entry = &self.trace_synchrotron()[trace_id];
                if !trace_cache_entry.expanded() {
                    trace_cache_entry.subtrace_ids()?;
                }
                Some(TraceSynchrotronToggleExpansion::new(trace_id).into())
            }
            TraceViewAction::Marker { _marker } => todo!(),
            TraceViewAction::ToggleAssocTrace {
                trace_id,
                assoc_trace_id,
            } => {
                if !self.trace_synchrotron().is_trace_cached(assoc_trace_id) {
                    return None;
                }
                Some(TraceSynchrotronAction::ToggleAssocTrace {
                    trace_id,
                    assoc_trace_id,
                })
            }
            TraceViewAction::FollowTrace { trace_id } => {
                let trace_synchrotron = self.trace_synchrotron();
                let accompanying_trace_ids = trace_synchrotron
                    .accompanying_trace_ids_except_followed()
                    .clone();
                let caryatid = trace_synchrotron.caryatid();
                let (has_figure, _) = trace_synchrotron.has_figure(
                    Some(trace_id),
                    caryatid.clone(),
                    accompanying_trace_ids,
                );
                if !has_figure {
                    return None;
                }
                Some(TraceSynchrotronAction::FollowTrace { trace_id })
            }
            TraceViewAction::ToggleAccompany { trace_id } => {
                {
                    // see if toggling accompany will affect figure
                    let trace_synchrotron = self.trace_synchrotron();
                    let caryatid = trace_synchrotron.caryatid();
                    let mut accompanying_trace_ids =
                        trace_synchrotron.accompanying_trace_ids().clone();
                    accompanying_trace_ids.toggle(trace_id);
                    let (has_figure, _) = trace_synchrotron.has_figure(
                        trace_synchrotron.followed_trace_id(),
                        caryatid.clone(),
                        AccompanyingTraceIdsExceptFollowed::new(
                            trace_synchrotron.followed_trace_id(),
                            accompanying_trace_ids,
                        ),
                    );
                    if !has_figure {
                        return None;
                    }
                }
                Some(TraceSynchrotronAction::ToggleAccompany { trace_id })
            }
            TraceViewAction::SetCaryatid { ref caryatid } => {
                let trace_synchrotron = self.trace_synchrotron();
                {
                    use crate::caryatid::IsCaryatid;
                    // see if setting pedestal will affect stalk
                    for trace_id in trace_synchrotron.trace_listing() {
                        let entry = &trace_synchrotron[trace_id];
                        if !entry.has_stalk(&caryatid.pedestal(todo!())) {
                            return None;
                        }
                    }
                }
                {
                    // see if setting pedestal will affect figure
                    let trace_synchrotron = self.trace_synchrotron();
                    let accompanying_trace_ids_expect_followed =
                        trace_synchrotron.accompanying_trace_ids_except_followed();
                    let (has_figure, _) = trace_synchrotron.has_figure(
                        trace_synchrotron.followed_trace_id(),
                        caryatid.clone(),
                        accompanying_trace_ids_expect_followed,
                    );
                    if !has_figure {
                        return None;
                    }
                }
                Some(TraceSynchrotronAction::SetCaryatid {
                    caryatid: caryatid.clone(),
                })
            }
        }
    }
}
