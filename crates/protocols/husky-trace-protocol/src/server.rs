use std::{
    collections::HashMap,
    net::{SocketAddr, ToSocketAddrs},
};

use crate::{
    id_map::TraceIdMap,
    message::{TraceRequest, TraceResponse},
    view::TraceViewData,
    *,
};
use husky_visual_protocol::{IsVisualComponent, IsVisualProtocol};
use husky_websocket_utils::easy_server::{easy_serve, IsEasyWebsocketServer};

pub struct TraceServer<Tracetime: IsTracetime> {
    cache: TraceCache<Tracetime::VisualComponent>,
    actions: Vec<TraceAction>,
    tracetime: Tracetime,
    traces: TraceIdMap<Tracetime::Trace>,
}

impl<Tracetime: IsTracetime> Default for TraceServer<Tracetime>
where
    Tracetime: Default,
{
    fn default() -> Self {
        Self {
            cache: Default::default(),
            actions: Default::default(),
            tracetime: Default::default(),
            traces: Default::default(),
        }
    }
}

impl<Tracetime: IsTracetime> TraceServer<Tracetime> {
    fn new(tracetime: Tracetime) -> Self {
        Self {
            cache: Default::default(),
            actions: Default::default(),
            tracetime,
            traces: Default::default(),
        }
    }

    fn init(&mut self) {
        let traces = self.tracetime.get_root_traces();
        for &trace in traces {
            todo!()
        }
        todo!()
    }
}

impl<Tracetime: IsTracetime> IsEasyWebsocketServer for TraceServer<Tracetime> {
    type Response = TraceResponse<Tracetime::VisualComponent>;

    type Request = TraceRequest;

    type SerdeImpl = Tracetime::SerdeImpl;

    fn handle(&mut self, request: Self::Request) -> Option<Self::Response> {
        match request {
            TraceRequest::Init => {
                self.init();
                todo!()
            }
        }
    }
}

pub trait IsTracetime: Send + 'static + Sized {
    type Trace: Send + Eq + std::hash::Hash + Copy;

    type VisualComponent: IsVisualComponent;

    type SerdeImpl: serde_impl::IsSerdeImpl;

    /// final
    fn serve_traces(self, addr: impl ToSocketAddrs) {
        TraceServer::new(self).easy_serve(addr)
    }

    fn get_root_traces(&self) -> &[Self::Trace];

    fn get_subtraces(&self, trace: Self::Trace) -> &[Self::Trace];

    fn get_trace_view_data(&self, trace: Self::Trace) -> &TraceViewData;
}
