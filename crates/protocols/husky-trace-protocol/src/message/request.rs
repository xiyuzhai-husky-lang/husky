use super::*;

/// message sent from trace client to trace server
#[derive(Debug, Serialize, Deserialize)]
pub enum TraceRequest {
    Init,
}
