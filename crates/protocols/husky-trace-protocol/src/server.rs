use std::net::SocketAddr;

use crate::{
    message::{TraceRequest, TraceResponse},
    view::TraceViewData,
    *,
};
use husky_visual_protocol::IsVisualProtocol;
use husky_websocket_utils::easy_server::{easy_serve, IsEasyWebsocketServer};

pub struct TraceServer<Tracetime: IsTracetime> {
    cache: TraceCache<Tracetime::VisualProtocol>,
    actions: Vec<TraceAction>,
    tracetime: Tracetime,
    traces: Vec<Tracetime::Trace>,
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
            traces: vec![],
        }
    }
}

impl<Tracetime: IsTracetime> TraceServer<Tracetime> {
    fn new(db: Tracetime) -> Self {
        Self {
            cache: Default::default(),
            actions: Default::default(),
            tracetime: db,
            traces: vec![],
        }
    }
}

impl<Tracetime: IsTracetime> IsEasyWebsocketServer for TraceServer<Tracetime> {
    type Response = TraceResponse<Tracetime::VisualProtocol>;

    type Request = TraceRequest;

    type SerdeImpl = Tracetime::SerdeImpl;

    fn handle(&mut self, request: Self::Request) -> Option<Self::Response> {
        match request {
            TraceRequest::Init => todo!(),
        }
    }
}

pub trait IsTracetime: Send + 'static + Sized {
    type Trace: Send + Copy;

    type VisualProtocol: IsVisualProtocol;

    type SerdeImpl: serde_impl::IsSerdeImpl;

    /// final
    fn serve_traces(self, addr: impl Into<SocketAddr>) {
        TraceServer::new(self).easy_serve(addr)
    }

    fn get_root_traces(&self) -> &[Self::Trace];

    fn get_subtraces(&self, trace: Self::Trace) -> &[Self::Trace];

    fn get_trace_view_data(&self, trace: Self::Trace) -> &TraceViewData;
}
