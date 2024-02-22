use self::action::TraceSynchrotronActionsDiff;
use super::*;

/// message sent from trace client to trace server
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraceResponse<TraceProtocol: IsTraceProtocol> {
    Init {
        trace_synchrotron: TraceSynchrotron<TraceProtocol>,
    },
    TakeTraceSynchrotronActionsDiff {
        trace_synchrotron_actions_diff: TraceSynchrotronActionsDiff<TraceProtocol>,
    },
    Err(String),
}
