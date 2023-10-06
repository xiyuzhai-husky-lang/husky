use super::*;

/// message sent from trace client to trace server
#[derive(Debug, Serialize, Deserialize)]
pub enum TraceClientMessage {
    Request(TraceClientRequest),
    Response(TraceClientResponse),
    Notification(TraceClientNotification),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TraceClientRequest {}

#[derive(Debug, Serialize, Deserialize)]
pub enum TraceClientResponse {}

#[derive(Debug, Serialize, Deserialize)]
pub enum TraceClientNotification {}
