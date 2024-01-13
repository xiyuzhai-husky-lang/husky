use std::net::ToSocketAddrs;

use crate::{
    center::action::{
        TraceSynchrotronNewTrace, TraceSynchrotronSetSubtraces, TraceSynchrotronToggleExpansion,
    },
    message::{TraceRequest, TraceResponse},
    view::{action::TraceViewAction, TraceViewData},
    *,
};
use husky_value_protocol::presentation::{ValuePresentationSynchrotron, ValuePresenterCache};
use husky_websocket_utils::easy_server::IsEasyWebsocketServer;

pub struct TraceServer<Tracetime: IsTracetime> {
    trace_synchrotron: Option<TraceSynchrotron<Tracetime::TraceProtocol>>,
    value_presenter_cache: ValuePresenterCache,
    tracetime: Tracetime,
}

impl<Tracetime: IsTracetime> Default for TraceServer<Tracetime>
where
    Tracetime: Default,
{
    fn default() -> Self {
        Self {
            trace_synchrotron: Default::default(),
            value_presenter_cache: Default::default(),
            tracetime: Default::default(),
        }
    }
}

impl<Tracetime: IsTracetime> TraceServer<Tracetime> {
    fn new(tracetime: Tracetime) -> Self {
        Self {
            trace_synchrotron: Default::default(),
            tracetime,
            value_presenter_cache: Default::default(),
        }
    }

    fn init(&mut self) {
        if self.trace_synchrotron.is_some() {
            return;
        }
        let traces = self.tracetime.get_root_traces();
        self.trace_synchrotron = Some(TraceSynchrotron::new(traces.iter().map(|&trace| {
            (
                trace.into(),
                self.tracetime.get_trace_view_data(trace).clone(),
            )
        })));
        self.cache_periphery()
    }

    #[track_caller]
    fn center(&self) -> &TraceSynchrotron<Tracetime::TraceProtocol> {
        self.trace_synchrotron.as_ref().unwrap()
    }

    #[track_caller]
    fn center_mut(&mut self) -> &mut TraceSynchrotron<Tracetime::TraceProtocol> {
        self.trace_synchrotron.as_mut().unwrap()
    }
}

impl<Tracetime> IsEasyWebsocketServer for TraceServer<Tracetime>
where
    Tracetime: IsTracetime,
    Tracetime::TraceProtocol: Serialize + for<'a> Deserialize<'a>,
{
    type Response = TraceResponse<Tracetime::TraceProtocol>;

    type Request = TraceRequest<Tracetime::TraceProtocol>;

    type SerdeImpl = Tracetime::SerdeImpl;

    fn handle(&mut self, request: Self::Request) -> Option<Self::Response> {
        match request {
            TraceRequest::Init {
                trace_protocol_type_name,
            } => {
                if trace_protocol_type_name != std::any::type_name::<Tracetime::TraceProtocol>() {
                    // todo: make this a type
                    return Some(TraceResponse::Err(format!(
                        r#"server's trace protocol is of type `{},
but client's trace protocol is of type `{trace_protocol_type_name}`."#,
                        std::any::type_name::<Tracetime::TraceProtocol>()
                    )));
                }
                self.init();
                let Some(cache) = self.trace_synchrotron.clone() else {
                    unreachable!()
                };
                Some(TraceResponse::Init { center: cache })
            }
            TraceRequest::TakeViewAction {
                view_action,
                synchrotron_actions_len,
            } => {
                use husky_print_utils::p;
                let Some(ref mut _cache) = self.trace_synchrotron else {
                    unreachable!()
                };
                assert_eq!(self.center().actions_len(), synchrotron_actions_len);
                self.take_view_action(view_action);
                let center_actions = self
                    .center()
                    .reproduce_synchrotron_actions(synchrotron_actions_len);
                p!(center_actions);
                Some(TraceResponse::TakeTraceSynchrotronAction { center_actions })
            }
            TraceRequest::NotifyViewAction { center_action, .. } => {
                self.center_mut().take_action(center_action);
                None
            }
        }
    }
}

impl<Tracetime: IsTracetime> TraceServer<Tracetime> {
    fn take_view_action(&mut self, view_action: TraceViewAction<Tracetime::TraceProtocol>) {
        match view_action {
            TraceViewAction::ToggleExpansion { trace_id } => {
                assert!(!self.center()[trace_id].expanded());
                // todo: handle more cases like subtraces with channels
                if self.center()[trace_id].subtrace_ids().is_some() {
                    return;
                }
                let subtraces = self.tracetime.get_subtraces(trace_id.into()).to_vec();
                let subtrace_ids = subtraces
                    .into_iter()
                    .map(|subtrace| {
                        let subtrace_id = subtrace.into();
                        self.cache_trace_if_new(subtrace_id);
                        subtrace_id
                    })
                    .collect();
                self.center_mut()
                    .take_action(TraceSynchrotronSetSubtraces::new(trace_id, subtrace_ids));
                self.center_mut()
                    .take_action(TraceSynchrotronToggleExpansion::new(trace_id))
            }
            TraceViewAction::Marker { _marker } => todo!(),
            TraceViewAction::ToggleAssociatedTrace {
                trace_id,
                associated_trace_id,
            } => {
                self.cache_trace_if_new(associated_trace_id);
                self.center_mut()
                    .take_action(TraceSynchrotronAction::ToggleAssociatedTrace {
                        trace_id,
                        associated_trace_id,
                    })
            }
        }
        self.cache_periphery()
    }

    fn cache_trace_if_new(&mut self, trace_id: TraceId) {
        if !self.center().is_trace_cached(trace_id) {
            let view_data = self.tracetime.get_trace_view_data(trace_id.into());
            self.center_mut()
                .take_action(TraceSynchrotronNewTrace::new(trace_id, view_data))
        }
    }

    fn cache_periphery(&mut self) {
        self.cache_stalks();
        self.cache_figure()
    }

    fn cache_stalks(&mut self) {
        let center = &self.center();
        let trace_listing = center.trace_listing();
        let pedestal = center.pedestal();
        for trace_id in trace_listing {
            self.cache_stalk(trace_id, pedestal)
        }
    }

    fn cache_stalk(
        &mut self,
        trace_id: TraceId,
        pedestal: <<Tracetime as IsTracetime>::TraceProtocol as IsTraceProtocol>::Pedestal,
    ) {
        if !self.center()[trace_id].has_stalk(pedestal) {
            let center = self.trace_synchrotron.as_mut().unwrap();
            let stalk = self.tracetime.get_trace_stalk(
                pedestal,
                trace_id.into(),
                &mut self.value_presenter_cache,
                center.value_presentation_synchrotron_mut(),
            );
            center.take_action(TraceSynchrotronAction::CacheStalk {
                pedestal,
                trace_id,
                stalk,
            });
        }
    }

    fn cache_figure(&mut self) {
        // todo!()
    }
}

pub trait IsTracetime: Send + 'static + Sized {
    type Trace: IsTrace;
    //  Send + Eq + std::hash::Hash + Copy;

    type TraceProtocol: IsTraceProtocolFull;

    type SerdeImpl: serde_impl::IsSerdeImpl;

    /// final
    fn serve_traces(self, addr: impl ToSocketAddrs) {
        TraceServer::new(self).easy_serve(addr)
    }

    fn get_root_traces(&self) -> &[Self::Trace];

    fn get_subtraces(&self, trace: Self::Trace) -> &[Self::Trace];

    fn get_trace_view_data(&self, trace: Self::Trace) -> TraceViewData;

    fn get_trace_stalk(
        &self,
        pedestal: <Self::TraceProtocol as IsTraceProtocol>::Pedestal,
        trace: Self::Trace,
        value_presenter_cache: &mut ValuePresenterCache,
        value_presentation_synchrotron: &mut ValuePresentationSynchrotron,
    ) -> TraceStalk;
}
