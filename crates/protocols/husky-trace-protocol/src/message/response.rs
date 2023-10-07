use super::*;
use crate::cache::TraceCache;
use std::{convert::Infallible, marker::PhantomData};

/// message sent from trace client to trace server
#[derive(Debug, Serialize, Deserialize)]
pub enum TraceResponse<VisualProtocol: IsVisualProtocol> {
    Init { cache: TraceCache<VisualProtocol> },
}
