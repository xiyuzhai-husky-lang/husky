use std::net::SocketAddr;

use crate::{
    message::{TraceRequest, TraceResponse},
    view::TraceViewData,
    *,
};
use husky_visual_protocol::IsVisualProtocol;
use husky_websocket_utils::easy_server::{easy_serve, IsEasyWebsocketServer};

pub struct TraceServer<Db: TraceServerDb> {
    cache: TraceCache<Db::VisualProtocol>,
    actions: Vec<TraceAction>,
    db: Db,
    traces: Vec<Db::Trace>,
}

impl<Db: TraceServerDb> Default for TraceServer<Db>
where
    Db: Default,
{
    fn default() -> Self {
        Self {
            cache: Default::default(),
            actions: Default::default(),
            db: Default::default(),
            traces: vec![],
        }
    }
}

impl<Db: TraceServerDb> TraceServer<Db> {
    fn new(db: Db) -> Self {
        Self {
            cache: Default::default(),
            actions: Default::default(),
            db,
            traces: vec![],
        }
    }
}

impl<Db: TraceServerDb> IsEasyWebsocketServer for TraceServer<Db> {
    type Response = TraceResponse<Db::VisualProtocol>;

    type Request = TraceRequest;

    type SerdeImpl = Db::SerdeImpl;

    fn handle(&mut self, request: Self::Request) -> Option<Self::Response> {
        match request {
            TraceRequest::Init => todo!(),
        }
    }
}

pub trait TraceServerDb: Send + 'static + Sized {
    type Trace: Send + Copy;

    type VisualProtocol: IsVisualProtocol;

    type SerdeImpl: serde_impl::IsSerdeImpl;

    /// final
    fn serve_traces(self, addr: impl Into<SocketAddr>) {
        TraceServer::new(self).easy_serve(addr)
    }

    fn get_root_traces(&self) -> &[Self::Trace];

    fn get_subtraces(&self) -> &[Self::Trace];

    fn get_trace_view_data(&self, trace: Self::Trace) -> &TraceViewData;
}
