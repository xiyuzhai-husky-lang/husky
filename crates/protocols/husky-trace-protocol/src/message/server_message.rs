use super::*;
use crate::cache::TraceCache;
use std::{convert::Infallible, marker::PhantomData};

/// message sent from trace client to trace server
#[derive(Debug, Serialize, Deserialize)]
pub enum TraceServerMessage<VisualProtocol: IsVisualProtocol> {
    Request(TraceServerRequest),
    Response(TraceServerResponse),
    Notification(TraceServerNotification<VisualProtocol>),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TraceServerRequest {}

#[derive(Debug, Serialize, Deserialize)]
pub enum TraceServerResponse {}

#[derive(Debug, Serialize, Deserialize)]
pub enum TraceServerNotification<VisualProtocol: IsVisualProtocol> {
    Init { cache: TraceCache<VisualProtocol> },
}
